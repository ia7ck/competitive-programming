import java.io.PrintWriter
import kotlin.math.min

fun main() {
    val pw = PrintWriter(System.out)
    val tt = readLine()!!.toInt()
    for (ii in 1..tt) {
        val s = readLine()!!.map { it.toInt() - '0'.toInt() }
        if (!s.contains(1) || !s.contains(2) || !s.contains(3)) {
            pw.println(0)
            continue
        }
        var freq = Array(4) {0}
        var l = 0
        var ans = s.size
        for (r in s.indices) {
            freq[s[r]] += 1
            while (freq.slice(1..3).all { it >= 1 }) {
                ans = min(ans, r - l + 1)
                freq[s[l]] -= 1
                l += 1
            }
        }
        pw.println(ans)
    }
    pw.flush()
}