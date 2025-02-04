const std = @import("std");
const mach = @import("mach");
const App = @import("App.zig");

const Modules = mach.Modules(.{
    mach.Core,
    App,
});

pub fn main() !void {
    const allocator = std.heap.c_allocator;

    var mods: Modules = undefined;
    try mods.init(allocator);

    const app = mods.get(.app);
    app.run(.main);
}
