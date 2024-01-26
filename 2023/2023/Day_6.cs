namespace _2023
{
    internal static class Day_6
    {
        private static readonly string inputPath = "./Inputs/Day_6.txt";
        private static readonly string testPath = "./Tests/Day_6.txt";

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);
            var times = ConvertLineToIntArray(lines[0]);
            var distances = ConvertLineToIntArray(lines[1]);
            int[] results = new int[times.Length];
            for (int i = 0; i < times.Length; i++)
            {
                results[i] = CalculateNumberOfWinningOptions(times[i], distances[i]);
            }

            int answer = 1;
            foreach (var item in results)
            {
                answer *= item;
            }

            Console.WriteLine($"Answer: {answer}");
        }

        private static int[] ConvertLineToIntArray(string line)
        {
            return Array.ConvertAll(line.Split(":")[1].Split(" ", StringSplitOptions.RemoveEmptyEntries), int.Parse);
        }

        private static int CalculateNumberOfWinningOptions(int time, int distance)
        {
            int minVelocity = 0;
            while (minVelocity * (time - minVelocity) <= distance)
            {
                minVelocity++;
            }
            return time - minVelocity * 2 + 1;
        }
    }
}
