namespace _2023
{
    internal static class Day_8
    {
        private static readonly string inputPath = "./Inputs/Day_8.txt";
        private static readonly string testPath = "./Tests/Day_8.txt";
        private static readonly string startingNode = "AAA";
        private static readonly string endingNode = "ZZZ";
        private static int Part;

        private record Node(string Key, string Left, string Right);

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);
            Part = part;

            var instructions = lines[0];
            var nodes = LoadNodes(lines[1..]);
            int steps = 0;
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
                var nodesString = parts[1].Substring(1, parts[1].Length - 2);
                var nodesPart = nodesString.Split(", ");
                var left = nodesPart[0];
                var right = nodesPart[1];
                dict.Add(key, new Node(key, left, right));
            }
            return dict;
        }

        private static int CountPathSteps(Dictionary<string, Node> nodes, string instructions, string startingNodeKey)
        {
            string currentNodeKey = startingNodeKey;
            int steps = 0;
            while (currentNodeKey != endingNode)
            {
                var node = nodes[currentNodeKey];
                int dir = instructions[steps % instructions.Length];
                if (dir == 'L')
                {
                    currentNodeKey = node.Left;
                }
                else
                {
                    currentNodeKey = node.Right;
                }
                steps++;
            }
            return steps;
        }

        //todo: its too slow :((
        private static int CountALlPathStepsConcurrently(Dictionary<string, Node> nodes, string instructions)
        {
            var currentNodesList = new List<string>();
            foreach (var node in nodes)
            {
                if (node.Key.EndsWith('A'))
                {
                    currentNodesList.Add(node.Key);
                }
            }

            int steps = 0;

            var currentNodes = currentNodesList.ToArray();

            bool shouldStop = false;
            while (!shouldStop)
            {
                shouldStop = true;
                int dir = instructions[steps % instructions.Length];
                for (int i = 0; i < currentNodes.Length; i++)
                {
                    var node = nodes[currentNodes[i]];
                    if (dir == 'L')
                    {
                        currentNodes[i] = node.Left;
                    }
                    else
                    {
                        currentNodes[i] = node.Right;
                    }
                    if (!currentNodes[i].EndsWith('Z'))
                    {
                        shouldStop = false;
                    }
                }
                steps++;
            }

            return steps;
        }
    }
}
