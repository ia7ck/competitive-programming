fun main() {
    val tt = readLine()!!.toInt()
    for (ii in 1..tt) {
        val (a, b, c, d) = readLine()!!.split(' ').map { it.toLong() }
        if (a <= b) {
            println(b)
            continue
        }
        if (c < d) {
            println(-1)
            continue
        }
        if (a > b && c <= d) {
            println(-1)
            continue
        }
        var k = (a - b) / (c - d)
        if ((a - b) % (c - d) != 0L) k += 1
        println(b + k * c)
    }
}