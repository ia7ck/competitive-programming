import java.util.*

fun main(args: Array<String>) {
    val (n, m) = readLine()!!.split(' ').map { it.toInt() }
    val a = readLine()!!.split(' ').map { it.toLong() }

    val q = PriorityQueue<Long>(n) { o1: Long, o2: Long -> -o1.compareTo(o2) }
    for (x in a) {
        q.add(x)
    }
    repeat(m) {
        val x = q.poll()
        q.add(x / 2)
    }
    val ans = q.sum()
    println(ans)
}