import Cocoa

let task = NSTask()
let bundle = NSBundle.mainBundle()
task.launchPath = bundle.pathForResource("webkitten-cocoa", ofType: nil)

// Set webkitten as the default browser:
//let bundleID = bundle.bundleIdentifier! as CFStringRef
//LSSetDefaultHandlerForURLScheme("http", bundleID);
//LSSetDefaultHandlerForURLScheme("https", bundleID);


task.launch()
repeat {
    sleep(1)
} while task.running