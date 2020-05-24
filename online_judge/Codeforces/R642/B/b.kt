fun main(args: Array<String>) {
    val tt = readLine()!!.toInt()
    repeat(tt) {
        val (n, k) = readLine()!!.split(' ').map { it.toInt() }
        val a = readLine()!!.split(' ').map { it.toInt() }.sorted().toTypedArray()
        val b = readLine()!!.split(' ').map { it.toInt() }.sorted().reversed().toTypedArray()
        var cnt = 0
        for ((j, x) in b.withIndex()) {
            val i = a.indexOfFirst { y -> y < x }
            if (i >= 0 && cnt < k) {
                cnt += 1
                val tmp = a[i]
                a[i] = b[j]
                b[j] = tmp
            }
        }
        println(a.sum())
    }
}