namespace _2023
{
    internal static class Day_3
    {
        private static readonly string inputPath = "./Inputs/Day_3.txt";
        private static readonly string testPath = "./Tests/Day_3.txt";

        private struct NumberInText
        {
            public int value;
            public int left;
            public int right;
            public int level;

            public NumberInText(int value, int left, int right, int level)
            {
                this.value = value;
                this.left = left;
                this.right = right;
                this.level = level;
            }
        }

        public static void Solve(bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Split("\n");
            for (int i = 0; i < lines.Length; i++)
            {
                lines[i] = lines[i].Replace("\r", String.Empty);
            }
            int sum = 0;
            for (int k = 0; k < lines.Length; k++)
            {
                var line = lines[k];
                for (int i = 0; i < line.Length; i++)
                {
                    int length = 0;
                    if (char.IsDigit(line[i]))
                    {
                        int first = i;
                        length = 1;
                        for (int j = i + 1; j < line.Length; j++)
                        {
                            if (!char.IsDigit(line[j]))
                            {
                                break;
                            }
                            length++;
                        }
                        int value = int.Parse(line.Substring(first, length));
                        var num = new NumberInText(value, i, i + length - 1, k);
                        if (IsPartNumber(lines, num))
                        {
                            sum += num.value;
                        }
                    }
                    i += length;
                }
            }

            Console.WriteLine($"Sum: {sum}");
        }

        private static bool IsPartNumber(string[] lines, NumberInText num)
        {
            int level = num.level;
            bool isLeftWall = num.left == 0;
            bool isRightWall = num.right == lines[level].Length - 1;
            int startLeft = isLeftWall ? num.left : num.left - 1;
            int rightEnd = isRightWall ? num.right : num.right + 1;

            if (!isLeftWall && IsCorrectSymbol(lines[level][num.left - 1]))
            {
                return true;
            }

            if (!isRightWall && IsCorrectSymbol(lines[level][num.right + 1]))
            {
                return true;
            }


            if (level > 0)
            {
                for (int i = startLeft; i <= rightEnd; i++)
                {
                    if (IsCorrectSymbol(lines[level - 1][i]))
                    {
                        return true;
                    }
                }
            }

            if (level < lines.Length - 1)
            {
                for (int i = startLeft; i <= rightEnd; i++)
                {
                    if (IsCorrectSymbol(lines[level + 1][i]))
                    {
                        return true;
                    }
                }
            }

            return false;
        }

        private static bool IsCorrectSymbol(char c)
        {
            return c != '.';
        }
    }
}
