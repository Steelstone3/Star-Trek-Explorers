namespace StarTrek.Contracts.Game
{
    public interface IGameState
    {
        void GoToState( IGameState theState );

        void StartState();

        void StopState();
    }
}