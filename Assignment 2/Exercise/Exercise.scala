/*
CMPT 383 Assignment 2
Exercise
Jiadi Luo
301354107
*/

import java.time.LocalDate

object Exercise {
    /******************** Exercise 2: the divisors, primes, join, and pythagorean functions ********************/
    // divisors
    def divisors(n: Int): List[Int] = {
        return (2 to n/2).filter(n % _ == 0).toList
    }

    // primes
    def primes(n: Int): List[Int] = {
        return (2 to n).filter(divisors(_).isEmpty).toList
    }

    // join
    def join(sep: String, a: List[String]): String = {
        if (a.isEmpty) {
            return ""
        } else if (a.length == 1) {
            return a.head
        } else {
            return a.head + sep + join(sep, a.tail)
        }
    }

    // pythagorean triples
    def pythagorean(n: Int): List[(Int, Int, Int)] = {
        val triples = for {
            c <- 1 to n
            b <- 1 to c
            a <- 1 to b
            if a * a + b * b == c * c
        } yield (a, b, c)

        return triples.toList
    }

    /******************** Exercise 3: mergeSort (and thus probably also merge), isPrimeDay, and isFriday ********************/
    // merge
    def merge(a: List[Int], b: List[Int]): List[Int] = {
        if (a.isEmpty) {
            return b
        } else if (b.isEmpty) {
            return a
        } else if (a.head < b.head) {
            return a.head :: merge(a.tail, b)
        } else {
            return b.head :: merge(a, b.tail)
        }
    }

    // mergeSort
    def mergeSort(a: List[Int]): List[Int] = {
        if (a.length <= 1) {
            return a
        } else {
            val (left, right) = a.splitAt(a.length / 2)
            return merge(mergeSort(left), mergeSort(right))
        }
    }

    // isFriday
    def isFriday(y: Int, m: Int, d: Int): Boolean = {
        val date = LocalDate.of(y, m, d)
        if (date.getDayOfWeek.toString == "FRIDAY") {
            return true
        } else {
            return false
        }
    }

    // isPrimeDay
    def isPrimeDay(y: Int, m: Int, d: Int): Boolean = {
        if (divisors(d).isEmpty) {
            return true
        } else {
            return false
        }
    }

    // testing
    def main(args: Array[String]): Unit = {
        // test divisors
        println("divisors(30) = " + divisors(30))
        println("divisors(64) = " + divisors(64))
        println("divisors(127) = " + divisors(127))

        // test primes
        println("primes(7) = " + primes(7))
        println("primes(100) = " + primes(100))

        // test join
        println("join(\",\", List(\"one\", \"two\", \"three\")) = " + join(", ", List("one", "two", "three")))
        println("join(\"+\", List(\"1\", \"2\", \"3\")) = " + join("+", List("1", "2", "3")))
        println("join(\"X\", List(\"abc\")) = " + join("X", List("abc")))
        println("join(\"X\", List()) = " + join("X", List()))

        // test pythagorean
        println("pythagorean(10) = " + pythagorean(10))
        println("pythagorean(30) = " + pythagorean(30))
        
        // test merge
        println("merge(List(2, 4, 6, 8), List(1, 3, 5, 7)) = " + merge(List(2, 4, 6, 8), List(1, 3, 5, 7)))
        println("merge(List(1, 2, 3, 4), List(5, 6, 7, 8, 9, 10)) = " + merge(List(1, 2, 3, 4), List(5, 6, 7, 8, 9, 10)))
        println("merge(List(4, 5, 7, 8), List(1, 2, 3, 6, 9)) = " + merge(List(4, 5, 7, 8), List(1, 2, 3, 6, 9)))

        // test merge sort
        println("mergeSort(List(1, 9, 3, 2, 7, 6, 4, 8, 5)) = " + mergeSort(List(1, 9, 3, 2, 7, 6, 4, 8, 5)))
        println("mergeSort(List(6, 2, 4, 8, 9, 5, 3, 1, 7, 10)) = " + mergeSort(List(6, 2, 4, 8, 9, 5, 3, 1, 7, 10)))
        println("mergeSort(List()) = " + mergeSort(List()))
        println("mergeSort(List(4)) = " + mergeSort(List(4)))
        println("mergeSort(List(4, 3)) = " + mergeSort(List(4, 3)))

        // test isFriday
        println("isFriday(2018, 5, 17) = " + isFriday(2018, 5, 17))
        println("isFriday(2018, 5, 18) = " + isFriday(2018, 5, 18))

        // test isPrimeDay
        println("isPrimeDay(2018, 5, 13) = " + isPrimeDay(2018, 5, 13))
        println("isPrimeDay(2018, 5, 14) = " + isPrimeDay(2018, 5, 14))
        println("isPrimeDay(2018, 6, 23) = " + isPrimeDay(2018, 6, 23))
    }
}
