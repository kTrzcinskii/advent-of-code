namespace _2023
{
    internal static class Day_2
    {
        private static readonly string inputPath = "./Inputs/Day_2.txt";
        private static readonly string testPath = "./Tests/Day_2.txt";
        private static readonly int MAX_RED = 12;
        private static readonly int MAX_GREEN = 13;
        private static readonly int MAX_BLUE = 14;

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Split('\n');

            int idSum = 0;
            int powerSum = 0;

            foreach (var line in lines)
            {
                bool possible = true;
                var parts = line.Split(": ");
                int id = int.Parse(parts[0].Split(" ")[1]);
                var sets = parts[1].Split("; ");
                int minBlue = -1, minRed = -1, minGreen = -1;
                foreach (var set in sets)
                {
                    var nums = set.Split(", ");

                    foreach (var num in nums)
                    {
                        var numberAndColor = num.Split(" ", StringSplitOptions.TrimEntries);
                        int count = int.Parse(numberAndColor[0]);
                        switch (numberAndColor[1])
                        {
                            case "blue":
                                if (count > MAX_BLUE && part == 1)
                                {
                                    possible = false;
                                }
                                if (count > minBlue && part == 2)
                                {
                                    minBlue = count;
                                }
                                break;
                            case "red":
                                if (count > MAX_RED && part == 1)
                                {
                                    possible = false;
                                }
                                if (count > minRed && part == 2)
                                {
                                    minRed = count;
                                }
                                break;
                            case "green":
                                if (count > MAX_GREEN && part == 1)
                                {
                                    possible = false;
                                }
                                if (count > minGreen && part == 2)
                                {
                                    minGreen = count;
                                }
                                break;
                        }
                        if (!possible && part == 1)
                        {
                            break;
                        }
                    }
                    if (!possible && part == 1)
                    {
                        break;
                    }
                }
                if (possible && part == 1)
                {
                    idSum += id;
                }
                if (part == 2)
                {
                    powerSum += minBlue * minGreen * minRed;
                }
            }

            if (part == 1)
            {
                Console.WriteLine($"Sum of ids: {idSum}");
            }
            if (part == 2)
            {
                Console.WriteLine($"Sum of powers: {powerSum}");
            }
        }
    }
}
