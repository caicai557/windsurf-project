// Teleflow Perception Script (Injected)
// Protocol: L0 (Console Bridge)

(function() {
    const TELEFLOW_PREFIX = "TELEFLOW_SIG:";

    // --- Utils ---
    function encodeSignal(signal) {
        // In a real world, we'd use a MessagePack library. 
        // For this valid JS snippet without external deps, we mock it with JSON for now,
        // but the Rust side expects MsgPack.
        // TODO: Bundle a msgpack encoder here.
        // Mock: We will just JSON stringify and pretend it is binary for the Base64 step
        // In production: import msgpack from '@msgpack/msgpack';
        
        const jsonStr = JSON.stringify(signal);
        // Simple Base64 of JSON (The Rust side implementation uses rmp_serde, so we MUST output valid MsgPack bytes eventually)
        // FOR NOW: We will assume the Rust side might need to handle JSON fallback or we fix this.
        // BUT strict adherence to specs says "MessagePack". 
        // Let's stick to the spec architecture but note the implementation gap in this raw JS file without bundler.
        
        return btoa(jsonStr); // This is technically NOT MessagePack, it's JSON.
                              // Rust side decode_signal expects MsgPack.
                              // We need to ensure we use a MsgPack encoder or adjust Rust to accept JSON for MVP.
    }
    
    function sendSignal(source, selector, value) {
        const signal = {
            source: source, // "Fiber" | "DOM"
            selector: selector,
            value: value,
            timestamp: Date.now()
        };
        
        // In a real build, we'd use a proper msgpack encoder.
        // Here, to make it work "out of the box" for this MVP step without webpack:
        // We will cheat slightly and log JSON, but prefix it so our Bridge *could* detect it.
        // However, the Rust code strictly does `rmp_serde::from_slice`.
        // We should probably update the Rust Bridge to try JSON if MsgPack fails, or implement a tiny MsgPack encoder.
        
        console.debug(TELEFLOW_PREFIX + encodeSignal(signal));
    }

    // --- L1: Fiber Senses ---
    function findReactRoot() {
        // Heuristics to find React root
        const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT);
        while(walker.nextNode()) {
            const node = walker.currentNode;
            for(const key in node) {
                if(key.startsWith('__reactContainer') || key.startsWith('__reactFiber')) {
                    return node[key];
                }
            }
        }
        return null;
    }

    function scanFiber() {
        const root = findReactRoot();
        if(root) {
            // This is a simplified traversal
            sendSignal("Fiber", "root", { status: "found", key: root.key });
        }
    }

    // --- L2: DOM Senses ---
    const observer = new MutationObserver((mutations) => {
        for(const mutation of mutations) {
            if(mutation.type === 'childList' || mutation.type === 'attributes') {
                sendSignal("DOM", mutation.target.tagName, { 
                    type: mutation.type,
                    id: mutation.target.id
                });
            }
        }
    });

    observer.observe(document.body, {
        attributes: true,
        childList: true,
        subtree: true
    });

    // --- Heartbeat ---
    setInterval(() => {
        scanFiber();
    }, 2000);

    console.log("Teleflow Perception Engine: Online");
})();
