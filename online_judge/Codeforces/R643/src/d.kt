fun main() {
    val (n, s) = readLine()!!.split(' ').map { it.toInt() }
    if (s < n * 2) {
        println("NO")
        return
    }
    println("YES")
    var a = Array(n) { 2 }
    a[0] += s - n * 2
    println(a.joinToString(" "))
    println(1)
}