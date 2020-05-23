import kotlin.math.PI
import kotlin.math.cos

fun main() {
    val tt = readLine()!!.toInt()
    repeat(tt) {
        val n = readLine()!!.toInt()
        val y = (360.0 / n / 2) * 2 * PI / 360
        var x = if (n / 2 % 2 == 1) y / 2 else 0.0
        var ans = 1.0
        repeat((n - 1) / 2) {
            x += (360.0 / n / 2) * 2 * PI / 360
            ans += cos(x) * 2
        }
        println("%.10f".format(ans))
    }
}