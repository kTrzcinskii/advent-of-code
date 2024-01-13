namespace _2023
{
    internal static class Day_1
    {
        private static readonly string inputPath = "./Inputs/Day_1.txt";
        private static readonly string testPath = "./Tests/Day_1.txt";

        private static readonly Dictionary<string, char> digits = new Dictionary<string, char>() { { "one", '1' }, { "two", '2' }, { "three", '3' }, { "four", '4' }, { "five", '5' }, { "six", '6' }, { "seven", '7' }, { "eight", '8' }, { "nine", '9' } };

        public static void Solve(bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Split("\n");
            int sum = 0;
            foreach (var line in lines)
            {
                char firstDigit, secondDigit;
                var characters = line.ToList();
                firstDigit = characters.Find(char.IsDigit);
                secondDigit = characters.FindLast(char.IsDigit);

                int minIndex = line.Length, maxIndex = -1;
                char minDigit = '\0', maxDigit = '\0';
                foreach (var digit in digits)
                {
                    if (line.Contains(digit.Key))
                    {
                        int idx = line.IndexOf(digit.Key);
                        if (idx < minIndex)
                        {
                            minIndex = idx;
                            minDigit = digit.Value;
                        }
                        idx = line.LastIndexOf(digit.Key);
                        if (idx > maxIndex)
                        {
                            maxIndex = idx;
                            maxDigit = digit.Value;
                        }
                    }
                }

                if (!line.Contains(firstDigit) || line.IndexOf(firstDigit) > minIndex)
                {
                    firstDigit = minDigit;
                }
                if (!line.Contains(secondDigit) || line.LastIndexOf(secondDigit) < maxIndex)
                {
                    secondDigit = maxDigit;
                }
                sum += int.Parse($"{firstDigit}{secondDigit}");
            }
            Console.WriteLine($"Sum = {sum}");
        }
    }
}
