namespace _2023
{
    internal static class Day_4
    {
        private static readonly string inputPath = "./Inputs/Day_4.txt";
        private static readonly string testPath = "./Tests/Day_4.txt";
        private static readonly int pointsBase = 2;

        public static void Solve(bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Split('\n');

            int sum = 0;

            foreach (var line in lines)
            {
                sum += CalculatePoints(line);
            }

            Console.WriteLine($"Sum: {sum}");
        }

        private static int CalculatePoints(string line)
        {
            var (winningNumbers, actualNumbers) = GetNumbers(line);

            int count = winningNumbers.Intersect(actualNumbers).Count();
            if (count == 0)
            {
                return 0;
            }

            return (int)Math.Pow(pointsBase, count - 1);
        }

        private static (int[] winningNumbers, int[] actualNumbers) GetNumbers(string line)
        {
            line = line.Substring(line.IndexOf(":") + 1);
            var sp = line.Split(" | ");
            var winningNumbers = Array.ConvertAll(sp[0].Trim().Split(" ", StringSplitOptions.RemoveEmptyEntries), int.Parse);
            var actualNumbers = Array.ConvertAll(sp[1].Trim().Split(" ", StringSplitOptions.RemoveEmptyEntries), int.Parse);
            return (winningNumbers, actualNumbers);
        }
    }
}
