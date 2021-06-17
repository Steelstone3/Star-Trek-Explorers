namespace StarTrek.Contracts.Character
{
    public interface ICrewRole
    {
        enum ShipRank
        {
            Captain,
            FirstOfficer,
            Commander,
            LieutenantCommander,
            Lieutenant,
            Ensign,
            Cadet,
        }

        string Rank{get;}
        string Role{get;}
    }
}