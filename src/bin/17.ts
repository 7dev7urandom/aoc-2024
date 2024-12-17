function check_n(a: bigint, nums: number[], l: number): bigint | false {
    for (let i = 0; i <= 7; i++) {
        if (verify(a + BigInt(i), nums.slice(l))) {
            if (nums.length == 0) {
                return a + BigInt(i);
            }
            if (l === 0) return a + BigInt(i);
            let res: bigint | false = check_n(
                (a + BigInt(i)) * BigInt(8),
                nums,
                l - 1
            );
            if (res) return res;
        }
    }

    return false;
}
function attempt(arr: number[]) {
    return check_n(BigInt(0), arr, arr.length - 1);
}

function verify(a: bigint, nums: number[]) {
    while (true) {
        let b = a % BigInt(8);
        b = b ^ BigInt(1);
        let c = a / BigInt(2) ** b;
        b = b ^ c;
        b = b ^ BigInt(4);
        a = a / BigInt(8);
        if (b % BigInt(8) !== BigInt(nums.shift() ?? -1)) return false;
        if (a === BigInt(0)) return true;
    }
}
let prog_array = `2,4,1,1,7,5,4,7,1,4,0,3,5,5,3,0`
    .split(",")
    .map((x) => parseInt(x));
let w = attempt(prog_array);
console.log(w, w ? verify(w, prog_array) : undefined);
