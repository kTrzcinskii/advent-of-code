namespace _2023
{
    internal static class Day_4
    {
        private static readonly string inputPath = "./Inputs/Day_4.txt";
        private static readonly string testPath = "./Tests/Day_4.txt";
        private static readonly int pointsBase = 2;

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Split('\n');

            int sum = 0;
            List<int> nextCardCopies = new List<int>();

            foreach (var line in lines)
            {
                if (part == 1)
                {
                    sum += CalculatePoints(line);
                }
                else if (part == 2)
                {
                    var copies = nextCardCopies.Count > 0 ? nextCardCopies.ElementAt(0) + 1 : 1;
                    if (nextCardCopies.Count > 0)
                    {
                        nextCardCopies.RemoveAt(0);
                    }
                    var (count, nextCopies) = GetCardCopies(line, copies);
                    sum += count;
                    var arr = nextCardCopies.ToArray();
                    if (arr.Length < nextCopies.Length)
                    {
                        Array.Resize(ref arr, nextCopies.Length);
                    }
                    for (int i = 0; i < nextCopies.Length; i++)
                    {
                        arr[i] += nextCopies[i];
                    }
                    nextCardCopies = arr.ToList();
                }
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

        private static (int cardCount, int[] nextCopies) GetCardCopies(string line, int copies)
        {
            var (winningNumbers, actualNumbers) = GetNumbers(line);

            int count = winningNumbers.Intersect(actualNumbers).Count();
            if (count == 0)
            {
                return (copies, Array.Empty<int>());
            }

            int[] arr = new int[count];

            for (int i = 0; i < arr.Length; i++)
            {
                arr[i] = copies;
            }

            return (copies, arr);

        }
    }
}
