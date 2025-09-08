import { useState } from "react";

export default function WinModal({ show, data, btns }) {
    if (!data || data.length === 0) return null;

    const [sortKey, setSortKey] = useState("name");
    const [sortAsc, setSortAsc] = useState(true);

    const sortedData = [...data].sort((a, b) => {
        if (sortKey === "name") {
            return sortAsc
                ? a.name.localeCompare(b.name)
                : b.name.localeCompare(a.name);
        } else {
            return sortAsc
                ? a[sortKey] - b[sortKey]
                : b[sortKey] - a[sortKey];
        }
    });

    const handleSort = (key) => {
        if (sortKey === key) {
            setSortAsc(!sortAsc); // Toggle direction
        } else {
            setSortKey(key);
            setSortAsc(true); // Default to ascending on new column
        }
    };


    // Dynamically get score field names (excluding `name`)
    const scoreKeys = Object.keys(data[0]).filter(key => key !== "name");

    const badgeColors = [
        "text-bg-danger",    // Red
        "text-bg-info",      // Teal/Cyan
        "text-bg-warning",   // Yellow
        "text-bg-primary",   // Blue
        "text-bg-success",   // Green
    ];

    const textColors = [
        "text-danger",   // Red
        "text-info",     // Teal/Cyan
        "text-warning",  // Yellow
        "text-primary",  // Blue
        "text-success",  // Green
    ];

    function findColor(i, arr = badgeColors) {
        return arr[i % arr.length]
    }


    return (
        <>
            <div
                className={`modal fade ${show ? "show" : ""}`}
                id="winModal"
                aria-hidden={!show}
                data-bs-backdrop="static"
                aria-labelledby="staticBackdropLabel"
                style={{ display: show ? "block" : "none" }}
            >
                <div className='modal-dialog modal-dialog-scrollable modal-dialog-centered'>
                    <div className='modal-content'>
                        <div className='modal-header'>
                            <h1 className="modal-title fs-4">Scoreboard:</h1>
                        </div>

                        <div className='modal-body'>
                            <ul className="list-group">
                                <li className="list-group-item d-flex justify-content-between align-items-center">
                                    <strong
                                        onClick={() => handleSort("name")}
                                        role="button"
                                        className="fw-bolder"
                                        title="Sort by name"
                                    >
                                        Name {sortKey === "name" ? (
                                            <i className={`bi ${sortAsc ? "bi-caret-up-fill" : "bi-caret-down-fill"}`} />
                                        ) : (
                                            <i className={`bi ${sortAsc ? "bi-caret-up" : "bi-caret-down"}`} />
                                        )}
                                    </strong>
                                    {scoreKeys.map((scoreKey, index) => (
                                        <span
                                            key={index}
                                            role="button"
                                            className={`fw-bolder ${findColor(index, textColors)}`}
                                            onClick={() => handleSort(scoreKey)}
                                            title={`Sort by ${scoreKey}`}
                                        >
                                            {scoreKey}
                                            {sortKey === scoreKey ? (
                                                <i className={`bi ${sortAsc ? "bi-caret-up-fill" : "bi-caret-down-fill"}`} />
                                            ) : (
                                                <i className={`bi ${sortAsc ? "bi-caret-up" : "bi-caret-down"}`} />
                                            )}
                                        </span>
                                    ))}
                                </li>
                                {sortedData.map((player, index) => (
                                    <li key={index} className="list-group-item d-flex justify-content-between align-items-center">
                                        <p className="m-0">{player.name}</p>
                                        {scoreKeys.map((scoreKey, i) => (
                                            <span
                                                key={i}
                                                className={`badge ${findColor(i)} rounded-pill`}
                                            >
                                                {player[scoreKey]}
                                            </span>
                                        ))}
                                    </li>
                                ))}
                            </ul>
                        </div>

                        <div className='modal-footer'>
                            {btns}
                        </div>
                    </div>
                </div>
            </div>

            {show && <div className="modal-backdrop fade show" />}
        </>
    );
}
