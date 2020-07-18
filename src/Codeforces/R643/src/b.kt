import java.io.PrintWriter

fun main() {
    val pw = PrintWriter(System.out)

    val tt = readLine()!!.toInt()
    repeat(tt) {
        val n = readLine()!!.toInt()
        val a = readLine()!!.split(' ').map { it.toInt() }
        var freq = Array<Int>(n + 1) { 0 }
        for (it in a) freq[it] += 1
        var ans = 0
        var r = 0
        for (i in 1..n) {
            ans += (freq[i] + r) / i
            r = (freq[i] + r) % i
        }
        pw.println(ans)
    }
    pw.flush()
}

// 1 1 1 1 2 2 2 3 3
// 2 2 2 5 5 5 5