// What the gallery's app delegate calls in gpui-mobile and the gallery's Rust library.
#import <Foundation/Foundation.h>
#include <stdbool.h>

// The gallery (mobile/gallery/src/lib.rs): opens its window.
void ely_gallery_ios_start(void);

// gpui-mobile (src/ios/ffi.rs).
void *gpui_ios_get_window(void);
bool gpui_ios_request_frame(void *window);
void gpui_ios_set_frame_waker(void *window, void (*waker)(void *context), void *context);
void gpui_ios_will_enter_foreground(void *app);
void gpui_ios_did_become_active(void *app);
void gpui_ios_will_resign_active(void *app);
void gpui_ios_did_enter_background(void *app);
void gpui_ios_will_terminate(void *app);
void gpui_ios_handle_open_url(void *url);
