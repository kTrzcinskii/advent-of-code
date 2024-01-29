namespace _2023
{
    internal static class Day_8
    {
        private static readonly string inputPath = "./Inputs/Day_8.txt";
        private static readonly string testPath = "./Tests/Day_8.txt";
        private static readonly string startingNode = "AAA";
        private static readonly string endingNode = "ZZZ";
        private static int Part;

        private class Node
        {
            public string Key { get; private set; }
            public Node? Left { get; set; }
            public Node? Right { get; set; }

            public Node(string Key, Node? Left, Node? Right)
            {
                this.Key = Key;
                this.Left = Left;
                this.Right = Right;
            }

            public Node? MakeMove(char direction)
            {
                if (direction == 'L')
                    return Left;
                return Right;
            }
        }

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);
            Part = part;

            var instructions = lines[0];
            var nodes = LoadNodes(lines[1..]);
            long steps;
            if (Part == 1)
            {
                steps = CountPathSteps(nodes, instructions, startingNode);
            }
            else
            {
                steps = CountALlPathStepsConcurrently(nodes, instructions);
            }

            Console.WriteLine($"Required steps: {steps}");
        }

        private static Dictionary<string, Node> LoadNodes(string[] lines)
        {
            var dict = new Dictionary<string, Node>();
            foreach (var line in lines)
            {
                var parts = line.Split(" = ");
                string key = parts[0];
                dict.Add(key, new Node(key, null, null));
            }

            foreach (var line in lines)
            {
                var parts = line.Split(" = ");
                var root = dict[parts[0]];
                var nodesString = parts[1].Substring(1, parts[1].Length - 2);
                var nodesPart = nodesString.Split(", ");
                var left = dict[nodesPart[0]];
                var right = dict[nodesPart[1]];
                root.Left = left;
                root.Right = right;
            }

            return dict;
        }

        private static int CountPathSteps(Dictionary<string, Node> nodes, string instructions, string startingNodeKey)
        {
            string currentNodeKey = startingNodeKey;
            int steps = 0;
            var node = nodes[currentNodeKey];
            while (node!.Key != endingNode)
            {
                char dir = instructions[steps % instructions.Length];
                node = node.MakeMove(dir);
                steps++;
            }
            return steps;
        }

        private static long CountALlPathStepsConcurrently(Dictionary<string, Node> nodes, string instructions)
        {
            var currentNodesList = new List<Node?>();
            foreach (var node in nodes)
            {
                if (node.Key.EndsWith('A'))
                {
                    currentNodesList.Add(node.Value);
                }
            }

            int steps = 0;

            var currentNodes = currentNodesList.ToArray();
            var stepsCount = new long[currentNodes.Length];

            int finished = 0;
            while (finished < currentNodes.Length)
            {
                char dir = instructions[steps % instructions.Length];
                steps++;
                for (int i = 0; i < currentNodes.Length; i++)
                {
                    var node = currentNodes[i];
                    if (node == null)
                    {
                        continue;
                    }
                    node = node.MakeMove(dir);
                    if (node!.Key.EndsWith('Z'))
                    {
                        stepsCount[i] = steps;
                        currentNodes[i] = null;
                        finished++;
                    }
                    else
                    {
                        currentNodes[i] = node;
                    }
                }
            }


            return LCM(stepsCount);
        }

        private static long GCD(long n1, long n2)
        {
            if (n2 == 0)
            {
                return n1;
            }
            else
            {
                return GCD(n2, n1 % n2);
            }
        }

        private static long LCM(long[] numbers)
        {
            return numbers.Aggregate((S, val) => S * val / GCD(S, val));
        }
    }
}


