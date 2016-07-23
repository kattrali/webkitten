import Cocoa

let task = NSTask()
let bundle = NSBundle.mainBundle()
task.launchPath = bundle.pathForResource("webkitten-cocoa", ofType: nil)

task.launch()
task.waitUntilExit()