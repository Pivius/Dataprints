import React, { useState, useEffect, useRef } from 'react';
import "./styles/dropdown.scss";
// option array variables 
// label: the text to display
// isDropdown: Is the option a dropdown, otherwise a button
// dropdownOptions: the options to display in the dropdown
// onClick: the function to call when the option is clicked
// separator: a separator to display below the option
// hotKey: the hotkey to display next to the option

const Dropdown = ({label, options}) => {
    const [isDropdownOpen, setDropdownOpen] = useState(false);
    const dropdownRef = useRef(null);
    
    const toggleDropdown = () => {
        setDropdownOpen(!isDropdownOpen);
    };
    
    const handleOptionClick = (option) => {
        if (option.onClick && typeof option.onClick === 'function') {
            option.onClick();
        }
    };

    const handleClickOutside = (event) => {
        if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
            setDropdownOpen(false);
        }
    };

    useEffect(() => {
        document.addEventListener('mousedown', handleClickOutside);
    
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);

    return (
        <div className="dropdown" ref={dropdownRef}>
            <button onClick={toggleDropdown}>{label}</button>
            {isDropdownOpen && (
                <ul className="dropdown-content">
                    {options.map((option, index) => (
                        <React.Fragment key={option.label}>
                            <li onClick={() => handleOptionClick(option)}>
                                {option.isDropdown ? (
                                    <Dropdown label={option.label} options={option.dropdownOptions} />
                                ) : !option.isDropdown ? (
                                    <button><div>{option.label}</div>{option.hotKey != null && (<div>{option.hotKey.toString()}</div>)}</button>
                                ) : (
                                    option.label
                                )}
                            </li>
                            {index < options.length - 1 && option.separator && <hr className="separator" />}
                        </React.Fragment>
                    ))}
                </ul>
            )}
        </div>
    );
};

export default Dropdown;