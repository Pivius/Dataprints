const search = document.getElementById('search');
let explorerContent = [];

search.addEventListener('input', (event) => {
    const value = event.target.value.toLowerCase();
});

export default function ObjectExplorer() {
    return (
        <div className="explorer">
            <h1>Explorer</h1>
            <input 
                type="text" 
                id="search" 
                name="search" 
                placeholder="Search..." 
            />
            <ul className="explorer-content">

            </ul>
        </div>
    )
}