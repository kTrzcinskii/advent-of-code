namespace _2023
{
    internal static class Day_9
    {
        private static readonly string inputPath = "./Inputs/Day_9.txt";
        private static readonly string testPath = "./Tests/Day_9.txt";

        enum Direction
        {
            Last,
            First
        }

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);

            long sum = 0;
            foreach (var line in lines)
            {
                sum += ExtrapolateNextValue(line, part == 1 ? Direction.Last : Direction.First);
            }

            Console.WriteLine($"Sum: {sum}");
        }

        private static int ExtrapolateNextValue(string line, Direction dir)
        {
            var arrays = new List<int[]>();
            var startingNumbers = ConvertLineToValueArray(line);
            arrays.Add(startingNumbers);
            while (!arrays.Last().All(x => x == 0))
            {
                arrays.Add(GetDifferences(arrays.Last()));
            }

            int prev = 0;
            while (arrays.Count > 0)
            {
                var arr = arrays.Last();
                if (dir == Direction.First)
                {
                    prev = -prev + arr[0];
                }
                else
                {
                    prev += arr[^1];
                }
                arrays.Remove(arr);
            }

            return prev;
        }

        private static int[] ConvertLineToValueArray(string line)
        {
            return Array.ConvertAll(line.Split(" "), int.Parse);
        }

        private static int[] GetDifferences(int[] array)
        {
            var result = new int[array.Length - 1];
            for (int i = 0; i < array.Length - 1; i++)
            {
                result[i] = array[i + 1] - array[i];
            }
            return result;
        }
    }
}
