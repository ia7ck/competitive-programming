fun main(args: Array<String>) {
    val tt = readLine()!!.toInt()
    repeat(tt) {
        val (n, m) = readLine()!!.split(' ').map { it.toInt() }
        when (n) {
            1 -> println(0)
            2 -> println(m)
            else -> println(m * 2)
        }
    }
}