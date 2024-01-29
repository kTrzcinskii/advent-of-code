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

        private readonly struct MapRange
        {
            readonly long left, right;
            public MapRange(long left, long right)
            {
                this.left = left;
                this.right = right;
            }

            public readonly bool IncludeValue(long value)
            {
                return value >= left && value <= right;
            }
        }


        private static readonly string inputPath = "./Inputs/Day_5.txt";
        private static readonly string testPath = "./Tests/Day_5.txt";
        private static readonly Dictionary<Maps, string> maps = new Dictionary<Maps, string>() { { Maps.SeedToSoil, "seed-to-soil map:" }, { Maps.SoilToFertilizer, "soil-to-fertilizer map:" }, { Maps.FertilizerToWater, "fertilizer-to-water map:" }, { Maps.WaterToLight, "water-to-light map:" }, { Maps.LightToTemperature, "light-to-temperature map:" }, { Maps.TemperatureToHumidity, "temperature-to-humidity map:" }, { Maps.HumidityToLocation, "humidity-to-location map:" } };
        private static readonly int mapCount = 7;
        private static string[] lines;

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);
            long minLocation = long.MaxValue;
            if (part == 1)
            {
                var seeds = Array.ConvertAll(lines[0].Split(": ")[1].Split(" "), long.Parse);
                foreach (long seed in seeds)
                {
                    long location = FindLocation(seed);
                    if (location < minLocation)
                    {
                        minLocation = location;
                    }
                }
            }
            else
            {
                var seeds = Array.ConvertAll(lines[0].Split(": ")[1].Split(" "), long.Parse);
                MapRange[] mapRanges = new MapRange[seeds.Length / 2];
                for (int i = 0, j = 0; i < mapRanges.Length; i++, j += 2)
                {
                    mapRanges[i] = new MapRange(seeds[j], seeds[j] + seeds[j + 1]);
                }

                long value = -1;
                long leftLocation = 0;
                while (FindRangeInclude(mapRanges, value) == -1)
                {
                    leftLocation *= 2;
                    value = FindSeed(leftLocation);
                }


                minLocation = leftLocation;
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

        private static long FindSeed(long location)
        {
            for (int i = mapCount - 1; i >= 0; i--)
            {
                var data = GetLines((Maps)i);
                foreach (var line in data)
                {
                    var numbers = Array.ConvertAll(line.Split(" "), long.Parse);
                    if (location < numbers[0] || location > numbers[0] + numbers[2])
                    {
                        continue;
                    }
                    long diff = location - numbers[0];
                    location = numbers[1] + diff;
                    break;
                }
            }
            return location;
        }

        private static int FindRangeInclude(MapRange[] ranges, long value)
        {
            int index = -1;
            for (int i = 0; i < ranges.Length; i++)
            {
                if (ranges[i].IncludeValue(value))
                {
                    index = i;
                    break;
                }
            }
            return index;
        }
    }
}
