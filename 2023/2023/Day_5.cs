namespace _2023
{
    internal static class Day_5
    {
        private enum Maps
        {
            SeedToSoil,
            SoilToFertilizer,
            FertilizerToWater,
            WaterToLight,
            LightToTemperature,
            TemperatureToHumidity,
            HumidityToLocation
        }

        private static readonly string inputPath = "./Inputs/Day_5.txt";
        private static readonly string testPath = "./Tests/Day_5.txt";
        private static readonly Dictionary<Maps, string> maps = new Dictionary<Maps, string>() { { Maps.SeedToSoil, "seed-to-soil map:" }, { Maps.SoilToFertilizer, "soil-to-fertilizer map:" }, { Maps.FertilizerToWater, "fertilizer-to-water map:" }, { Maps.WaterToLight, "water-to-light map:" }, { Maps.LightToTemperature, "light-to-temperature map:" }, { Maps.TemperatureToHumidity, "temperature-to-humidity map:" }, { Maps.HumidityToLocation, "humidity-to-location map:" } };
        private static readonly int mapCount = 7;
        private static string[] lines;

        public static void Solve(bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);
            var seeds = Array.ConvertAll(lines[0].Split(": ")[1].Split(" "), long.Parse);

            long minLocation = long.MaxValue;
            foreach (long seed in seeds)
            {
                long location = FindLocation(seed);
                if (location < minLocation)
                {
                    minLocation = location;
                }
            }

            Console.WriteLine($"Min location: {minLocation}");
        }

        private static string[] GetLines(Maps map)
        {
            int startIndex = -1;
            for (int i = 0; i < lines.Length; i++)
            {
                if (lines[i] == maps[map])
                {
                    startIndex = i + 1;
                    break;
                }
            }
            if (startIndex == -1)
            {
                throw new ArgumentException("Couldn't find map name in lines");
            }

            int endIndex = -1;
            for (int i = startIndex; i < lines.Length; i++)
            {
                if (!char.IsDigit(lines[i][0]))
                {
                    endIndex = i;
                    break;
                }
            }
            if (endIndex == -1)
            {
                endIndex = lines.Length;
            }

            string[] result = new string[endIndex - startIndex];
            for (int i = 0; i < result.Length; i++)
            {
                result[i] = lines[startIndex + i];
            }

            return result;
        }

        private static long FindLocation(long seed)
        {
            for (int i = 0; i < mapCount; i++)
            {
                var data = GetLines((Maps)i);
                foreach (var line in data)
                {
                    var numbers = Array.ConvertAll(line.Split(" "), long.Parse);
                    if (seed < numbers[1] || seed > numbers[1] + numbers[2])
                    {
                        continue;
                    }
                    long diff = seed - numbers[1];
                    seed = numbers[0] + diff;
                    break;
                }
            }
            return seed;
        }
    }
}
