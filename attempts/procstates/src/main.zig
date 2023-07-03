const std = @import("std");
const jp = @import("2bitjmpproc.zig");


fn userget() !void {
    var prev: u16 = 0;
    while (true) {
        var innum: u16 = 0;
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        defer _ = gpa.deinit();
        const allocator = gpa.allocator();

        const stdin = std.io.getStdIn();

        std.debug.print("input: ", .{});
        const clinput = try stdin.reader().readUntilDelimiterAlloc(allocator, '\n', 1024);
        defer allocator.free(clinput);
        innum = std.fmt.parseInt(u16, clinput, 0) catch {
            prev = jp.getsingle(prev);
            continue;
        };
        prev = jp.getsingle(innum);
    }
}

const VisitedRet = struct {
    id: u16,
    allvisited: bool,
};
fn getFirstUnvisitedId(visited: *[(0xFFFF+1)]u1) VisitedRet {
    for(0..(0xFFFF+1)) |aid| {
        if(visited[aid] == 0){
            const aidv = VisitedRet{.id = @intCast(u16, aid),.allvisited = false};
            return aidv;
        }
    }
    const allvisited = VisitedRet{.id = @intCast(u16, 0xFFFF),.allvisited = true};
    return allvisited;
}

var path: [(0xFFFF+1)]u16 = [1]u16{0} ** (0xFFFF+1);
var pathlen: u16 = 0;
var pvisited: [(0xFFFF+1)]u1 = [1]u1{0} ** (0xFFFF+1);

var bestpath: [(0xFFFF+1)]u16 = [1]u16{0} ** (0xFFFF+1);
var bestpathlen: u16 = 0;
var bpvisited: [(0xFFFF+1)]u1 = [1]u1{0} ** (0xFFFF+1);

// fn longest_path() void{
//     // iterate after each command to get path until loop to find longest
//     var prev: u16 = 0;
//     var gpa = std.heap.GeneralPurposeAllocator(.{}){};
//     defer _ = gpa.deinit();
//     const allocator = gpa.allocator();
//     var alpath = std.ArrayList(u16).init(allocator);
//     var visited = std.ArrayList(u16).init(allocator);
//     while (true) {
//         var innum: u16 = 0;
//
//         alpath.resize(0);
//         prev = getsingle(prev);
//         std.mem.indexOfScalar(,,) orelse {
//             innum = getFirstUnvisitedId(, );
//             prev = getsingle(innum);
//             continue;
//         };
//     }
// }

var gvisited:  [(0xFFFF+1)]u1 = [1]u1{0} ** (0xFFFF+1);

fn iteratePath(start: u16) void {
    // Don't forget to @memset to 0 path and pvisited before this
    @memset(path[0..], 0);
    @memset(pvisited[0..], 0); // Why this makes any changes?

    pathlen = 0;
    var cs: u16 = start; // состояние
    var ns: u16 = undefined; // следующее состояние
    while(true) {
        // Ищем есть ли цикл.
        // Для этого смотрим не пройден ли уже
        // нынешний индекс.
        var looped = false;
        if(pathlen>0) {
            if(pvisited[cs] == 1){
                // So no need to add anything to path.
                looped = true;
//                 std.debug.print("next:{any} ", .{getNext(cs)});
//                 break;
            }
        }
        if(looped) {
            return;
        } else {
//             std.log.debug("cs:{d} ", .{cs});
            pathlen += 1;
            ns = jp.getNext(cs);
//             std.log.debug("ns:{d} ", .{ns});
            path[cs] = ns;
            gvisited[cs] = 1;
            pvisited[cs] = 1;
            cs = ns;
        }
    }
}

fn iterateAllPaths() void {
    for(4..20) |i| {
        iteratePath(@intCast(u16, i));
//         std.debug.print("\n", .{});
        std.debug.print("path{d}:\n",.{i});
        for(0..0xFFFF) |ii|{
            if(pvisited[ii]==1) {
                std.debug.print("{d} ", .{path[ii]});
            }
        }
        std.debug.print("\n", .{});
    }
}

fn longestPath() void {
    // DFS among all possible

    var cs: u16 = 0; // current state
    while (true) {
        iteratePath(cs);
        if(pathlen >= bestpathlen) {
            @memcpy(bestpath[0..], path[0..]);
            @memcpy(bpvisited[0..], pvisited[0..]);
            bestpathlen = pathlen;
            std.debug.print("new bestpath{d}:\n",.{cs});
            for(0..0xFFFF) |ii|{
                if(pvisited[ii]==1) {
                    std.debug.print("{d} ", .{path[ii]});
                }
            }
            std.debug.print("\n", .{});
        }
        const vret = getFirstUnvisitedId(&gvisited);
        if(vret.allvisited) {
            std.debug.print("FINAL bestpath:\n",.{});
            for(0..0xFFFF) |ii|{
                if(bpvisited[ii]==1) {
                    std.debug.print("{d} ", .{ii});
                }
            }
            std.debug.print("FINAL length:{}\n",.{bestpathlen});
            std.debug.print("\n", .{});
            break;
        }
        cs = vret.id;
    }
}

pub fn main() !void {
    std.time.sleep(1000000000000000);
//     try userget();
    longestPath();
//     iterateAllPaths();
}
