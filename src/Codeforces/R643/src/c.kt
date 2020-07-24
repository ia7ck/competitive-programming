fun main() {
    val (a, b, c, d) = readLine()!!.split(' ').map { it.toInt() }
    var s = Array(d * 2 + 2) { 0.toLong() }
    for (x in a..b) {
        s[x + b]++
        s[x + c + 1]--
    }
    for (i in 1 until s.size) {
        s[i] += s[i - 1]
    }
    for (i in 1 until s.size) {
        s[i] += s[i - 1]
    }
    var ans: Long = 0
    for (z in c..d) {
        ans += s.last() - s[z]
    }
    println(ans)
}