namespace _2023
{
    internal static class Day_3
    {
        private static readonly string inputPath = "./Inputs/Day_3.txt";
        private static readonly string testPath = "./Tests/Day_3.txt";
        private static readonly char ratioSymbol = '*';

        private struct ElementInText
        {
            public int left;
            public int right;
            public int level;

            public ElementInText(int left, int right, int level)
            {
                this.left = left;
                this.right = right;
                this.level = level;
            }
        }

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Replace("\r", string.Empty).Split("\n");
            if (part == 1)
            {
                Part1(lines);
            }
            else
            {
                Part2(lines);
            }
        }

        private static void Part1(string[] lines)
        {
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
                        var num = new ElementInText(i, i + length - 1, k);
                        if (IsPartNumber(lines, num))
                        {
                            sum += value;
                        }
                    }
                    i += length;
                }
            }

            Console.WriteLine($"Sum: {sum}");
        }

        private static void Part2(string[] lines)
        {
            int sum = 0;
            for (int k = 0; k < lines.Length; k++)
            {
                var line = lines[k];
                for (int i = 0; i < line.Length; i++)
                {
                    if (line[i] == ratioSymbol)
                    {
                        sum += FindRatio(lines, new ElementInText(i, i, k));
                    }
                }
            }
            Console.WriteLine($"Sum: {sum}");
        }

        private static bool IsPartNumber(string[] lines, ElementInText num)
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

        private static int FindRatio(string[] lines, ElementInText symbol)
        {
            int level = symbol.level;
            bool isLeftWall = symbol.left == 0;
            bool isRightWall = symbol.right == lines[level].Length - 1;
            int startLeft = isLeftWall ? symbol.left + 1 : symbol.left - 1;
            int rightEnd = isRightWall ? symbol.right - 1 : symbol.right + 1;

            int firstNumber = 0, secondNumber = 0;

            if (!isLeftWall && char.IsDigit(lines[level][symbol.left - 1]))
            {
                firstNumber = FindNumberInText(lines[level], symbol.left - 1, out _);
            }

            if (!isRightWall && char.IsDigit(lines[level][symbol.right + 1]))
            {
                int num = FindNumberInText(lines[level], symbol.right + 1, out _);
                if (firstNumber == 0)
                {
                    firstNumber = num;
                }
                else
                {
                    secondNumber = num;
                }
            }

            if (level > 0)
            {
                for (int i = startLeft; i <= rightEnd; i++)
                {
                    int step = 0;
                    if (char.IsDigit(lines[level - 1][i]))
                    {
                        int num = FindNumberInText(lines[level - 1], i, out step);
                        if (firstNumber == 0)
                        {
                            firstNumber = num;
                        }
                        else if (secondNumber == 0)
                        {
                            secondNumber = num;
                        }
                        else
                        {
                            return 0;
                        }
                    }
                    i += step;
                }
            }

            if (level < lines.Length - 1)
            {
                for (int i = startLeft; i <= rightEnd; i++)
                {
                    int step = 0;
                    if (char.IsDigit(lines[level + 1][i]))
                    {
                        int num = FindNumberInText(lines[level + 1], i, out step);
                        if (firstNumber == 0)
                        {
                            firstNumber = num;
                        }
                        else if (secondNumber == 0)
                        {
                            secondNumber = num;
                        }
                        else
                        {
                            return 0;
                        }
                    }
                    i += step;
                }
            }

            Console.WriteLine($"Znalezione numery: {firstNumber} - {secondNumber}");

            return firstNumber * secondNumber;
        }

        private static int FindNumberInText(string line, int start, out int step)
        {
            int len = 1;
            int i = start - 1;
            int leftLen = 0;
            while (i >= 0)
            {
                if (char.IsDigit(line[i]))
                {
                    leftLen++;
                    i--;
                }
                else
                {
                    break;
                }
            }
            int rightLen = 0;
            i = start + 1;
            while (i < line.Length)
            {
                if (char.IsDigit(line[i]))
                {
                    rightLen++;
                    i++;
                }
                else
                {
                    break;
                }
            }
            len += leftLen + rightLen;

            step = rightLen;

            return int.Parse(line.Substring(start - leftLen, len));
        }
    }
}
