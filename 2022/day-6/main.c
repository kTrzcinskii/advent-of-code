#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#define DISTINCT_CHAR 14

int main()
{ 
    int i, j, k;
    bool all_diff = false;
    char last[DISTINCT_CHAR] = {'\0'};
    int counter = 0;
    char* input = "sbpbwwrlwrwggscsfcsshsvhshphttzctztfflddvbvcbcvbcbnbrrstsspsvpvllncccqssqdssnsmshmshhtssbzzlplffzppddmhhwnhntnjtnjtnnqggjdgjdjrjffsdfsfbsffhtthwwfssqpspllcvcdvvfvfzfbzzgpglpgpmppfsfdfbfvvfjfsjjvqqbvqbvqbvvlvglvglvljvjzvvqcvqcqjcqqpddqbqcqrqmmvnvbnnfqnncznccqjcqqjzzwlltrrmmlwlllbjlblzlddfdmmscmsspfssqggbsbsjjdqdqgqgqbqdqrqlrrltttghthrtrwrdrnrfnfvftvtsvtvwtvwttvqtvtccqtqmqggwhhhlzhhrwwbwqwbbfrrmddwhdwdnwdwbdwwsswtwnnvvdggbbtwbbwllgffqpffpgpgmpmmjqqmpmffjgfgrffdzzspzptzzdszszbsbsbsvsbbhsbsbddnhhrqqcwwblbwlwnnthtsshmshmhgmgqgjqqpccfvcvbbbnllgmlgmmbnmnlmlzlffrrrgssmcmddnpdndtntrrqdqldldbbtppvddgndnwwctwccpbpffngnsgshggphggtssgngtnthtllzflzflzfzrfftgtstppghgnnpggrdgdnnswwccljlflwwgzgcgrrhssbwbllblnlvlddsffgrrnbbmsmmmhjmjvmjvmjmqqpdqppltlflmfmssqcqsqvsvsggpglplslggqtggmsmmmgffrccrvvsdvdvfddprdprrwjrjcjjmlmrmqmggqgdqdwdsdwwwgfwfddrcdcvcmmjbbmrrhlrrwcwvcclttwbwttgffnmnjmjdmmmlqmqrrlblplccdbcbwcbcpphpqpqlqrllqwwjsjgjbjqbbmcmzmrrqtrqqmcmmnjmjbbtnnbbjdjzzvbzvzjzdjzjrzzfzfbbmnmdnnzppmbpbhbdbvvqnqqnqrnrzrhhrddbqdqpqqwgwlglplglslhhpjpdjdjgddnppmvppjddczccgsccdsccbcvczccnbnmmsnsmmrwmwbmbbcpbbwbppzggdnnnzmnzzvrvcrvcvcrvrnrzzqzmzttjhhnffqggnqqwzwfzfwzwfwcwnccwgcwgcgqcgqcggzgwgfwgfffjttjddwswqqnttqwqsqhhcmchcscbbnwnlwlrlppqnnwjnjtntrtdtbbmzzrmzrzvrzvrzzwqwqllccnffvmmfmvmzzfttnnzttrbttdvvhdhzhzhvzzfsfszsvsttrqqdlldflfjjnqjjbhhqhzhcchppzsznntdntdddqqwjqjqmmgnnhrhjhnjnznwwdpplzzfzztbzzbmzmbbgjgtjtqjqpqhqgqhhtztpzpmzmjmhhjwwjsspbssvdsvsgvssghgvglgtltgllmhmtmhttfgglwgllpvpgpspzzhnnzcnzzldzdmzmlzzwtwhhlfhhvthhjdjsdsmddgzzsjjnznjnmjjgdgtdtzdtdvtddzwzgwwqvqccwrrrvddbtdbbshbhssnvsnvssswcwjcjlltvvltvvdsdqdsswlwttzfzhzjjgjsgjgttbccsvcscllbfbzzwwwznwwtzwzjzwjjgqjqrjrdjrddzsznsznzpzszhzzspzzbzrzhhsbspsqpqqqpddhdcdldttlccjrjsrsfftfdfrrtntpnpsnnwjnwnznpzzlqqlmqlmmcddlqqzssglsglssprmnvltqhslvqmvszjtvtwqjcdngjmftnhwvjdvtwwhtnsdmvjdspnhnlmjgnmwlspcvpdmlsrnbbzlmwwrslssmcbggmfvgzsnpnlnzdqsbhcfjdccrspnzfmhbvwstvccvqqjlwhpnlrrwszjnrtdfzwrwlzwvdvbzbvltdpfwrjlslmrctwvbbvdrctgtgwtwpjjghhvdsqhplfrsjqlgsrbfgwdjlzdpdljtvjmpwqqbghndqnvjhngtpnpvzfbtchncwdqjhmzjlpdggbdcqrfjlwvczvpspljqmpgtrsvwwhqncfvfrwbnvsfjqlsjdlrqzmlqjgcpghhgzfhjcglllnhtmchrrptbzhqnfntgqbfstrvpsqsvqvcvpgjnchbvmgtgzqfrqcjrvldzghdfrvllrtfcwnsmmgdrcbjmdqgbfwmpwhfjmnbqrbvqvnmjlqtsjqvhzpdsgbjpjngmzbgnznvjqprfvwzjrfrwfdqrtlcgqqlvrzwmqwjbdvprvpwvdcrfdrcttnmnvjfrsrrmjgfjdmpdpnfsrwnprtdpvdmdwssvjrqtlcvpgrqgqqqffvvssbmghzjrzrzlcrnfjtdbvwsjzfvcrsmgqbcrdrjwdwbltffwbgjwtgtdblmlhvhlcpgdmpcztmpmgjghqpwzwtpnmnmgnqqrrtwczgmgtdbgdqtpnlbnzhshsfzsmrztffrmlsgqcprbjpqwjlqgwvctpmpshgzbzsjgqhzvsrjfwplvjbvltrlfldvsmlppcmsfrbbctggmqmjnhppstzrcfjtdgfwrrnmlvjphwtlqtjqcntjtzvgjtwvthjfbgpwhlrzdqncmggvgthmgjvrbnzbndsldnlcgtcqbqdnnbnqhvtpfnrclttfwcpnqscjbzdvbqrrsbzpdfhjllbjwsltjpmdnbrrzvhvzzqnlbglsjrnjbqffnnzmldfvtvmldfsztrnpcjgblsdhzfzmfqzlfrtslglhfvszppptjnjqcdmjcwqmfzhnqbfhslwhvtjfvcftzsphvghvtjjswpwghnfngmzddbwwqddsphvhcrwtthsjfswfqbvdsqghmrspdldfqmchnnrcdvjsclcnlsncsplchvzrwqbtvvqlqspftrwmjcbgpzbsmnfccbzgnhqsfjgmmsqsscdscfjbrmmtjbsphhlrlsgbllrptqrcgnqchzfddjwlldsbpcnzfbspfpchclqfbbtjpmtmtjthcdvwhrtqbgmgldcgcnmmhtbnqpzzcwlrscbzcqjzgztwjrnbmsnqtcllznlctzrntftspmnvhtfwbljnmrwsstvbmwclqrfpmwvjphrwddzdwtlfcvzlvqdmzhnvslfnfjhvdndlgbvvzbztpwvqzbzsbtqpqfmgqfgpzctfrqfjwmsnmlfqbgrlmncntcbshjhdcbqnvznhtcgcmmhnsbwpzbvtqbntwgflhjgmvvfhdbwfqmnfjlzdvvnpmvjrdfdnrhpbtllhbtbswwvrbwjgnqpbgnfrjtvbczbpmrcbwdlhztzssnwshjmmcqchptrtchrqncdgdtmwrlnmmwqlzqswwwvpngvwcphgnzrhpprjnbldscvwlqdjwnhjrnscdwlnhnsbwgzjtgvzdgqcjcgvrdhntszhdnjsbbrfphlmdlldjdslbjnnsfbmcnvtlczmtnhrwblnbrdptcpmsbwqptgmwzsqnmmchwnnrvrlfsrglfzzqbnzmpdtnhhbmfqvsrsdsctvhqwfgbtvhbbrsrqmrvvplrnbfnbdmrvzpgctdtglndhcqnllvvcppgfbwjrpqcbghlqdbmpzwrqpmvwddqgthlmzmdsvzdfsmgzltbsvphctzgjsmqvgjlsbgnvmgprbcsfhgrtbwtnnrsqcwfzrhlgjcwcfrjhffrvrvtnpczvwvjnnhfdgcppnnjjpttptcbmdqvgdbhdmlqgcqsrnbcrbtcgzbvgmhbnwzsgnwzbhdqqmvtpssvlvsttgnmcclqnjcgjnvtdggrcwsgbpjljgzgtllsnfvfshtbbpwrjhzvzswlfdvhbpngvgddcmhbzqcvnjhfsqpnvvsdvdtmqlqpzcgsnwlflnqprbqnwdqchjvsptbtrvtzvhrmrvznfpzmcsgnqtdvghhzwrrwvqwrztvdbjjtfchpftdcbthpfdczwchpptwzdpswvbhppdphgvpfzhprpqtnprgfmdnqrbrdlclcmhrdfrcdhwpcqhnbwmhrrgnctpvsqmphcwwvlmslszhdz";

    for (i = 0; i < strlen(input); i++)
    {
        all_diff = true;
        last[i % DISTINCT_CHAR] = input[i];
        counter++;
        if (counter < DISTINCT_CHAR) 
        {
            all_diff = false;
            continue;
        }
        for(j = 0; j < DISTINCT_CHAR - 1; j++)
            for (k = j + 1; k < DISTINCT_CHAR; k++) if (last[k] == last[j]) all_diff = false;
        if (all_diff) break;
    }

    printf("result: %d\n", counter);

    return 0;
}