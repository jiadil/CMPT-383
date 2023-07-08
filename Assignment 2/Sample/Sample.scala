/*
CMPT 383 Assignment 2
Sample programs
Jiadi Luo
301354107
*/

import scala.collection.mutable.Queue

object Sample {
    /******************** Sample 1 ********************/
    /* In Sample 1, I will compare Scala with Haskell. Haskell has pattern matching and intuitive recursive usage, which makes it easier to write for some straightforward algorithms like this.
    One of the courses I'm taking introduces the concept of "Hill Climbing", which refers to a CSP that we consistently take steps towards improving the current solution. 
    However, it can be hindered by local maxima, which are peaks that may not be the highest point in the entire solution space.
    Therefore, I want to write a program that can find all peak values given an array.
    For example: 
    [1, 2, 3, 2, 1] has one peak: 3
    [1, 2, 3, 4, 5] has one peak: 5
    [3, 4, 3, 2, 1, 2] has two peaks: 4, 2
    [1, 1, 1, 1, 1] has no peaks
    */
    def findAllPeaks(a: Array[Int]): List[Int] = {
        // use a list to store the result
        var peaksResult = List.empty[Int] /* Compare with Haskell: 
                                            Haskell can use pattern matching to solve this problem in a more intuitive way. 
                                            Scala needs to use a list to store the result, which is more verbose and error-prone. */
        val length = a.length

        if (length > 0) {
            if (a(0) > a(1)) {
                peaksResult = a(0) :: peaksResult
            }

            for (i <- 1 until length - 1) {
                if (a(i) > a(i - 1) && a(i) > a(i + 1)) { /* Compare with Haskell: 
                                                            Haskell just recursively calls the function itself, which is more intuitive.
                                                            Scala needs to use a for loop to iterate through the array, which is more verbose and error-prone. */
                    peaksResult = a(i) :: peaksResult
                }
            }

            if (a(length - 1) > a(length - 2)) {
                peaksResult = a(length - 1) :: peaksResult
            }
        }

        return peaksResult.reverse
    }
    /* We can see that this program is easier to write in Haskell compared to Scala. The recursive usage and pattern matching in Haskell makes the code more concise, readable, and neat.
        findPeaks :: [Int] -> [Int]
        findPeaks [] = []
        findPeaks [x] = []
        findPeaks [x, y] = []
        findPeaks (x:y:z:rest)
            | y > x && y > z = y : findPeaks (y:z:rest)
            | otherwise = findPeaks (y:z:rest)
    */


    /******************** Sample 2 ********************/
    /* In Sample 2, I will still compare Scala with Haskell, but in a good way. While Haskell has a heavy use of monads to handle encapsulation, Scala has a more intuitive way to do it, which is by using "Option".
    This program creates a database of students and their numbers, and then prints the number of a given student.
    for example, if the database is:
        Alex: 1
        Bob: 2
        Clare: 3
    printStudentNumber("Alex") should print "Alex's number is 1"
    printStudentNumber("David") should print "David is not in the database"
    */
    // declare a case class to store the student's name and number
    case class Student(name: String, number: Int)
    class StudentDatabase {
        // create a map to store the students
        private val students = Map(
            "Alex" -> Student("Alex", 1),
            "Bob" -> Student("Bob", 2),
            "Clare" -> Student("Clare", 3)
        )

        def findStudentNumber(name: String): Option[Int] = { /* Compare with Haskell: 
                                                                Scala doesn't need to worry about anything, the language will handle the encapsulation for us.
                                                                In Haskell, we need to manually use monads to handle encapsulation, which is more verbose and error-prone. */
            students.get(name).map(_.number) /* Compare with Haskell: 
                                                Scala can just get the number!
                                                In Haskell, we need to use Just & Nothing, which is more troublesome. */
        }
    }
    def printStudentNumber(name: String, database: StudentDatabase): Unit = {
        val number = database.findStudentNumber(name)
        number match {
            case Some(n) => println(s"$name's number is $n")
            case None => println(s"$name is not in the database") /* Compare with Haskell: 
                                                                    In Scala, we don't need to worry about anything.
                                                                    In Haskell, we still need to handle the output manually. */
        }
    }
    /* We can see that this program is harder to write in Haskell compared to Scala. Scala's Option makes the code more concise, readable, and neat.
        data Student = Student { name :: String, number :: Int }
        data StudentDatabase = StudentDatabase { students :: Map String Student }
        findStudentNumber :: String -> StudentDatabase -> Maybe Int
        findStudentNumber name database = case Map.lookup name (students database) of
            Just student -> Just (number student)
            Nothing -> Nothing
        printStudentNumber :: String -> StudentDatabase -> IO ()
        printStudentNumber name database = case findStudentNumber name database of
            Just number -> putStrLn (name ++ "'s number is " ++ show number)
            Nothing -> putStrLn (name ++ " is not in the database")
    */


    /******************** Sample 3 ********************/
    /* In Sample 3, I will compare Scala with C++. Scala has some powerful built-in functionality and libraries, which makes it easier to write some programs compared to C++.
    This sample is about finding the largest value in each level of a binary tree.
    For example:
            1*
           / \
          6*   2
         /    / \
        4    7   8*
    The result should be [1, 6, 8]
    */
    class TreeNode(var value: Int = 0, var left: TreeNode = null, var right: TreeNode = null)
    def findMaxEachLevel(root: TreeNode): Array[Int] = {
        // check if the root is null
        if (root == null) {
            return Array.empty[Int]
        }

        // use a mutable buffer to store the nodes in each level
        val result = scala.collection.mutable.ArrayBuffer[Int]() /* Compare with C++: 
                                                                    Scala uses built-in functionality and higher-level abstractions to make the code more concise and readable compared to 
                                                                    C++, which is more verbose and error-prone. */
        
        val queue = Queue[TreeNode]() /* Compare with C++: 
                                        Scala's Queue is a generic class, which means it can be used for any type of data. 
                                        C++'s queue is a template class, which means it can only be used for one type of data. */

        queue.enqueue(root)

        while (queue.nonEmpty) {
            val levelSize = queue.size
            var maxVal = Int.MinValue

            // iterate through the nodes in the current level
            for (_ <- 0 until levelSize) {
                val node = queue.dequeue()

                // update the max value
                maxVal = Math.max(maxVal, node.value)

                if (node.left != null) {
                    queue.enqueue(node.left)
                }
                if (node.right != null) {
                    queue.enqueue(node.right)
                }
            }

            result += maxVal
        }

        return result.toArray /* Compare with C++: 
                                Scala's toArray() method is a built-in method, which means it's more concise and readable.
                                In C++, we need to write our own function to convert a vector to an array, which is more verbose and error-prone. */
    }


    /******************** Sample 4 ********************/
    /* At last, in this sample, I would like to include something interesting and interactive which can read user input.
    This is called "Guess game", which I've learned before in CMPT 225.
    The game is simple: the computer will think of a number between 1 and 100, and the player will guess the number.
    The computer will tell the player whether the guess is too high or too low.
    The player will keep guessing until he/she gets the right answer.
    */
    def guessGame = {
        val number = scala.util.Random.nextInt(100) + 1
        var guess = 0
        var count = 0
        while (guess != number) {
            print("Please enter your guess: ")
            guess = scala.io.StdIn.readInt()
            count += 1
            if (guess > number) {
                println("Too high!")
            } else if (guess < number) {
                println("Too low!")
            } else {
                println("You got it!")
            }
        }
        println("You have guessed " + count + " times.")
    }


    /******************** Testing ********************/
    def main(args: Array[String]): Unit = {
        // test Sample 1: findAllPeaks
        println("findAllPeaks([1, 2, 3, 2, 1]) = " + findAllPeaks(Array(1, 2, 3, 2, 1)))
        println("findAllPeaks([1, 2, 3, 4, 5]) = " + findAllPeaks(Array(1, 2, 3, 4, 5)))
        println("findAllPeaks([3, 4, 3, 2, 1, 2]) = " + findAllPeaks(Array(3, 4, 3, 2, 1, 2)))
        println("findAllPeaks([1, 1, 1, 1, 1]) = " + findAllPeaks(Array(1, 1, 1, 1, 1)))

        // test Sample 2: printStudentNumber
        val database = new StudentDatabase()
        printStudentNumber("Alex", database)
        printStudentNumber("Bob", database)
        printStudentNumber("Clare", database)
        printStudentNumber("David", database) // shouldn't exist

        // test Sample 3: findMaxEachLevel
        val root = new TreeNode(1)
        root.left = new TreeNode(6)
        root.left.left = new TreeNode(4)
        root.right = new TreeNode(2)
        root.right.left = new TreeNode(7)
        root.right.right = new TreeNode(8)
        println("findMaxEachLevel(root) = " + findMaxEachLevel(root).mkString(", "))

        // test Sample 4: guessGame
        guessGame
    }
}
