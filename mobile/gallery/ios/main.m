// The gallery's iOS host: starts the Rust gallery full screen and drives its frames with a
// display link, as gpui-mobile's example host does.

#import <UIKit/UIKit.h>
#import "gpui_ios.h"

@interface ElyGalleryDelegate : UIResponder <UIApplicationDelegate>
@property (nonatomic, assign) void *gpuiWindow;
@property (nonatomic, strong) CADisplayLink *displayLink;
@end

static void ElyGalleryResumeFrames(void *context);

@implementation ElyGalleryDelegate

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    // A cold launch from gpui://gallery/<slug> hands its link here, before openURL: the
    // gallery reads it while opening its window.
    NSURL *link = launchOptions[UIApplicationLaunchOptionsURLKey];
    if (link) {
        gpui_ios_handle_open_url((__bridge void *)link.absoluteString);
    }
    ely_gallery_ios_start();
    self.gpuiWindow = gpui_ios_get_window();
    if (!self.gpuiWindow) {
        NSLog(@"Ely Gallery: no GPUI window was created");
        return YES;
    }
    [self startFrames];
    return YES;
}

// GPUI says after each frame whether it wants another; the link pauses in between and the
// waker resumes it, so an idle screen costs no CPU.
- (void)startFrames {
    self.displayLink = [CADisplayLink displayLinkWithTarget:self selector:@selector(renderFrame)];
    [self.displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
    gpui_ios_set_frame_waker(self.gpuiWindow, ElyGalleryResumeFrames, (__bridge void *)self);
}

- (void)renderFrame {
    if (self.gpuiWindow && !gpui_ios_request_frame(self.gpuiWindow)) {
        self.displayLink.paused = YES;
    }
}

static void ElyGalleryResumeFrames(void *context) {
    ElyGalleryDelegate *delegate = (__bridge ElyGalleryDelegate *)context;
    delegate.displayLink.paused = NO;
}

- (void)applicationWillEnterForeground:(UIApplication *)application {
    gpui_ios_will_enter_foreground(NULL);
    if (!self.displayLink && self.gpuiWindow) {
        [self startFrames];
    }
}

- (void)applicationDidBecomeActive:(UIApplication *)application {
    gpui_ios_did_become_active(NULL);
}

- (void)applicationWillResignActive:(UIApplication *)application {
    gpui_ios_will_resign_active(NULL);
}

- (void)applicationDidEnterBackground:(UIApplication *)application {
    gpui_ios_did_enter_background(NULL);
    [self.displayLink invalidate];
    self.displayLink = nil;
}

- (BOOL)application:(UIApplication *)application openURL:(NSURL *)url options:(NSDictionary<UIApplicationOpenURLOptionsKey, id> *)options {
    gpui_ios_handle_open_url((__bridge void *)url.absoluteString);
    return YES;
}

- (void)applicationWillTerminate:(UIApplication *)application {
    [self.displayLink invalidate];
    self.displayLink = nil;
    gpui_ios_will_terminate(NULL);
}

@end

int main(int argc, char *argv[]) {
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass([ElyGalleryDelegate class]));
    }
}
