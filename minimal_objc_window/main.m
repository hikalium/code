// Based on https://stackoverflow.com/questions/30269329/creating-window-application-in-pure-c-on-mac-osx/30269738

#import <Cocoa/Cocoa.h>
int main ()
    {
        @autoreleasepool{
            [NSApplication sharedApplication];
            [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];
            id applicationName = [[NSProcessInfo processInfo] processName];
            id window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 120, 120)
                styleMask:NSTitledWindowMask backing:NSBackingStoreBuffered defer:NO];
            [window cascadeTopLeftFromPoint:NSMakePoint(20,20)];
            [window setTitle: applicationName];
            [window makeKeyAndOrderFront:nil];
            [NSApp activateIgnoringOtherApps:YES];
            [NSApp run];
        }
        return 0;
}
