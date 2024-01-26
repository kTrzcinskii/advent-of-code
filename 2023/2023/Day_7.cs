namespace _2023
{
    internal static class Day_7
    {
        private static readonly string inputPath = "./Inputs/Day_7.txt";
        private static readonly string testPath = "./Tests/Day_7.txt";
        private static int Part;

        private struct Hand : IComparable<Hand>
        {
            public enum Type
            {
                HighCard,
                OnePair,
                TwoPair,
                ThreeOfAKind,
                FullHouse,
                FourOfAKind,
                FiveOfAKind
            }

            public string Cards { get; private set; }
            public int Bid { get; private set; }

            public Type CardsType { get; private set; }

            public Hand(string cards, int bid)
            {
                Cards = cards;
                Bid = bid;
                CardsType = CardsToType(cards);
            }

            public static explicit operator Hand(string input)
            {
                var s = input.Split(" ");
                return new Hand(s[0], int.Parse(s[1]));
            }

            public readonly int CompareTo(Hand other)
            {
                if (CardsType != other.CardsType)
                {
                    return CardsType.CompareTo(other.CardsType);
                }

                for (int i = 0; i < Cards.Length; i++)
                {
                    if (Cards[i] != other.Cards[i])
                    {
                        return CardToValue(Cards[i]).CompareTo(CardToValue(other.Cards[i]));
                    }
                }

                return 0;
            }

            private static Type CardsToType(string cards)
            {
                var counter = new Dictionary<char, int>();
                int specialBonus = 0;
                for (int i = 0; i < cards.Length; i++)
                {
                    if (Part == 2 && cards[i] == 'J')
                    {
                        specialBonus++;
                        continue;
                    }
                    if (counter.ContainsKey(cards[i]))
                    {
                        counter[cards[i]]++;
                    }
                    else
                    {
                        counter.Add(cards[i], 1);
                    }
                }

                if (counter.Count == 0)
                {
                    // only Js in the cards
                    return Type.FiveOfAKind;
                }

                var key = counter.Aggregate((x, y) => x.Value > y.Value ? x : y).Key;
                counter[key] += specialBonus;

                var values = counter.Values;
                if (counter.Count == 1)
                {
                    return Type.FiveOfAKind;
                }

                if (counter.Count == 2)
                {
                    if (values.Contains(4))
                    {
                        return Type.FourOfAKind;
                    }
                    if (values.Contains(3))
                    {
                        return Type.FullHouse;
                    }
                }

                if (counter.Count == 3)
                {
                    if (values.Contains(3))
                    {
                        return Type.ThreeOfAKind;
                    }
                    return Type.TwoPair;
                }

                if (counter.Count == 4)
                {
                    return Type.OnePair;
                }

                return Type.HighCard;
            }

            private static int CardToValue(char card)
            {
                if (char.IsDigit(card))
                {
                    return card - '2';
                }

                return card switch
                {
                    'T' => 8,
                    'J' => Part == 1 ? 9 : -1,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => throw new ArgumentException($"Character *{card}* is not a valid card symbol!"),
                };
            }
        }

        public static void Solve(int part = 1, bool isTest = false)
        {
            string input = File.ReadAllText(isTest ? testPath : inputPath);
            var lines = input.Replace("\r", string.Empty).Split("\n", StringSplitOptions.RemoveEmptyEntries);
            Part = part;

            Hand[] hands = new Hand[lines.Length];
            for (int i = 0; i < hands.Length; i++)
            {
                hands[i] = (Hand)lines[i];
            }

            Array.Sort(hands);

            int answer = 0;
            for (int i = 0; i < hands.Length; i++)
            {
                answer += hands[i].Bid * (i + 1);
            }

            Console.WriteLine($"Answer: {answer}");
        }
    }
}
