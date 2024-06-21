use yew::prelude::*;

#[function_component(Analyses)]
pub fn analyses() -> Html {
    html! {
        <div class="analyses">
            <div class="card">
                <h2>{ "Search Analyses" }</h2>
                <form>
                    <input type="text" placeholder="Filter by name" />
                    <button type="submit">{ "Search" }</button>
                </form>
            </div>
            <div class="styled-table">
                <h2>{ "Analyses" }</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Name" }</th>
                            <th>{ "Date" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{ "1" }</td>
                            <td>{ "Analysis 1" }</td>
                            <td>{ "2024-06-03" }</td>
                            <td>
                                <button onclick={Callback::from(|_| { /* Eliminar file */ })}>
                                    { "🗑️ Delete" }
                                </button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
