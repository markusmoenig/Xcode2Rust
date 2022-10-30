//
//  GameViewController.swift
//  Xcode2Rust macOS
//
//  Created by Markus Moenig on 16/10/22.
//

import Cocoa
import MetalKit

// Our macOS specific view controller
class GameViewController: NSViewController {

    var renderer: Renderer!
    var mtkView:  RMTKView!

    override func viewDidLoad() {
        super.viewDidLoad()

        guard let mtkView = self.view as? RMTKView else {
            print("View attached to GameViewController is not an MTKView")
            return
        }

        // Select the device to render with.  We choose the default device
        guard let defaultDevice = MTLCreateSystemDefaultDevice() else {
            print("Metal is not supported on this device")
            return
        }

        mtkView.device = defaultDevice

        renderer = Renderer(metalKitView: mtkView)

        let t = shipping_rust_addition(30, 1);

        print(t);
        
        let result = rust_greeting("Markus")
        let swift_result = String(cString: result!)
        print(swift_result)
        rust_greeting_free(UnsafeMutablePointer(mutating: result))
        
        renderer.mtkView(mtkView, drawableSizeWillChange: mtkView.drawableSize)

        mtkView.delegate = renderer        
    }
}
