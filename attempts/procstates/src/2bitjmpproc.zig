const std = @import("std");


var mem: u16 = 0;
var mem2: u16 = 0;
var outs: [(0xFFFF+1)]u16 = [1]u16{0} ** (0xFFFF+1);



const ProcessorState = packed struct {
    i0: u2,
    a0: u2,
    i1: u2,
    a1: u2,
    i2: u2,
    a2: u2,
    i3: u2,
    a3: u2,
};
const ProcessorMem = packed struct {
    v0: u4,
    v1: u4,
    v2: u4,
    v3: u4,
};

// Get instruction from ProcessorState
pub fn getpsi(buf: u16, id: u2) u2 {
    const cs: ProcessorState = @bitCast(ProcessorState, buf);
    //     std.debug.print("getpsi:{any}\n",.{cs});
    switch (id) {
        0 => return cs.i0,
        1 => return cs.i1,
        2 => return cs.i2,
        3 => return cs.i3,
    }
}
// Get argument from ProcessorState
pub fn getpsa(buf: u16, id: u2) u2 {
    const cs: ProcessorState = @bitCast(ProcessorState, buf);
    //     std.debug.print("getpsa:{any}\n",.{cs});
    switch (id) {
        0 => return cs.a0,
        1 => return cs.a1,
        2 => return cs.a2,
        3 => return cs.a3,
    }
}
// Get value from ProcessorMem
pub fn getpsv(buf: u16, id: u2) u4 {
    const cs: ProcessorMem = @bitCast(ProcessorMem, buf);
    //     std.debug.print("getpsv{any}\n",.{cs});
    switch (id) {
        0 => return cs.v0,
        1 => return cs.v1,
        2 => return cs.v2,
        3 => return cs.v3,
    }
}
// Set value to ProcessorMem
pub fn setpsv(buf2: *u16, id: u2, v: u4) void {
    const cs: *ProcessorMem = @ptrCast(*ProcessorMem, buf2);
    //     std.debug.print("v={} id={}\nsetpsv was:{any}\n",.{v, id, cs});
    switch (id) {
        0 => cs.v0 = v,
        1 => cs.v1 = v,
        2 => cs.v2 = v,
        3 => cs.v3 = v,
    }
    //     std.debug.print("setpsv now:{any}\n",.{cs});
}

// instructions
const Op = enum {
    put,
    loop,
    inc,
    dec,
};

var accum: u4 = 0;
const Putstate = enum { load, store };
var putstate = Putstate.load;
// Get value from location on one invoke, set value to location on another
pub fn put(buf: u16, buf2: *u16, arg: u2) void {
    if (putstate == Putstate.load) {
        accum = getpsv(buf, arg);
        putstate = Putstate.store;
    } else {
        putstate = Putstate.load;
        setpsv(buf2, arg, accum);
    }
}
// ++
pub fn inc(buf: u16, buf2: *u16, arg: u2) void {
    var added: u4 = 0;
    added = @addWithOverflow(getpsv(buf, arg), 1)[0];
    setpsv(buf2, arg, added);
}
// --
pub fn dec(buf: u16, buf2: *u16, arg: u2) void {
    var subbed: u4 = 0;
    subbed = @subWithOverflow(getpsv(buf, arg), 1)[0];
    setpsv(buf2, arg, subbed);
}

pub fn execute(buf: u16) u16 {
    var buf2 = buf;
    var pc: u2 = 0;
    var jumps = [4]i32{ 0, 0, 0, 0 };
    {
        // get jumps once, so no infinite loops possible
        var i: u2 = 0;
        while (true) {
            var ins = getpsi(buf, i);
            var arg = getpsa(buf, i);
            if (ins == 1) {
//                 std.debug.print("buf:{b} ins:{b}\n", .{ buf, ins });
                var jc: i32 = @intCast(i32, arg) + 1; // 0 loops are useless
                if (i > 0) {
                    var ii = i - 1;
                    while (true) {
                        jc *= jumps[ii]; // loop times loop
                        if (ii == 0) break;
                        ii -= 1;
                    }
                }
                jumps[i] = jc;
            }
            if (i == 3) break;
            i += 1;
        }
    }
//     std.debug.print("jumps:{any}\n", .{ jumps });
    // Надо вернуться к j по
    var j: u8 = 0;
    var exect = true;
    while (exect) {
        while (true) {
            var ins = getpsi(buf, pc);
            var arg = getpsa(buf, pc);
            switch (ins) {
                0 => {
//                     std.debug.print("put:{}\n", .{arg});
//                     std.debug.print("{b}=>", .{buf2});
                    put(buf, &buf2, arg);
//                     std.debug.print("{b}\n", .{buf2});
                },
                1 => {},
                2 => {
//                     std.debug.print("inc:{}\n", .{arg});
//                     std.debug.print("{b}=>", .{buf2});
                    inc(buf, &buf2, arg);
//                     std.debug.print("{b}\n", .{buf2});
                },
                3 => {
//                     std.debug.print("dec:{}\n", .{arg});
//                     std.debug.print("{b}=>", .{buf2});
                    dec(buf, &buf2, arg);
//                     std.debug.print("{b}\n", .{buf2});
                },
            }
            if (pc == 3) break;
            pc += 1;
        }
        // Тут проверяем нужен ли jmp. Начинаем от 0. Если jumps[j]>0, то уменьшаем число и возвращаемся к исполнению.
        // Если jumps[j]<=0, то j++ и если j<4 повторяем цикл проверки.
        while (j < 4) {
            if (jumps[j] > 0) {
                jumps[j] -= 1;
                break;
            } else {
                j += 1;
            }
        } else {
            //             std.debug.print("ex\n", .{});
            exect = false;
        }
    }
    putstate = Putstate.load;

    accum = 0;
    return buf2;
}

pub fn loopall() void {
    var i: u16 = 0;
    while (i < (0xFFFF+1)) {
        //         mem2 = execute(mem);
        //         outs[mem]=mem2;
        //         mem=mem2;
        mem2 = execute(i);
        outs[i] = mem2;
        std.log.debug("{b}to{b}\n", .{ i, mem2 });
        std.time.sleep(100000000);
        i += 1;
    }
    std.log.debug("{any}", .{outs});
}

pub fn getsingle(num: u16) u16 {
    mem2 = execute(num);
    outs[num] = mem2;
    std.log.info("{b}=>{b}", .{ num, mem2 });
    return mem2;
}

pub fn getNext(num: u16) u16 {
    mem2 = execute(num);
    outs[num] = mem2;
    return mem2;
}
