fun main() {
    val t = readLine()!!.toInt()
    repeat(t) {
        var (a, k) = readLine()!!.split(' ').map { it.toLong() }
        for (i in 2..k) {
            val ds = a.toString().toCharArray().map { it.toInt() - '0'.toInt() }
            val mn = ds.min()!!
            val mx = ds.max()!!
            if (mn == 0) break
            a += mn * mx
        }
        println(a)
    }
}