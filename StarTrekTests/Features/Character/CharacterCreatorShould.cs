using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Controllers.Game.Character.Factories;
using Xunit;

namespace StarTrekTests.Features.Character
{
    public class CharacterCreatorShould
    {
        private CharacterFactory _characterFactory = new CharacterFactory();
        
        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void CreateACaptain(string name)
        {
            var crewRole = new CaptainRole();

            var captain = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(captain);
            Assert.Equal(crewRole.Rank, captain.CrewRole.Rank);
            Assert.Equal(crewRole.Role, captain.CrewRole.Role);
            Assert.Equal(name, captain.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheCaptainToAStarship(string name)
        {
            var captainRole = new CaptainRole();
            var captain = new CrewMember(name, captainRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, captain);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.Captain, captain);
        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Captain", "Captain", "Bobus", "Bobbington")]
        public void CreateAFirstOfficer(string rank, string position, string firstName, string secondName)
        {

        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Captain", "Captain", "Bobus", "Bobbington")]
        public void CreateAHeadOfEngineering(string rank, string position, string firstName, string secondName)
        {

        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Captain", "Captain", "Bobus", "Bobbington")]
        public void CreateAHeadOfSecurity(string rank, string position, string firstName, string secondName)
        {

        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Captain", "Captain", "Bobus", "Bobbington")]
        public void CreateAHeadOfScience(string rank, string position, string firstName, string secondName)
        {

        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Captain", "Captain", "Bobus", "Bobbington")]
        public void CreateAHeadOfMedical(string rank, string position, string firstName, string secondName)
        {

        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Captain", "Captain", "Bobus", "Bobbington")]
        public void CreateAHeadOfTactical(string rank, string position, string firstName, string secondName)
        {

        }
    }
}