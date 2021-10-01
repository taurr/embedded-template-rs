switch variable::get("board") {
    "Microbit" => {
        file::delete("target-crates/board");
    }
    "Custom" => {
        file::delete("target-crates/nrf51.svd");
    }
}
