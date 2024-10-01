import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { AuthContext } from '../contexts/AuthContext';
import { FaBook, FaUser, FaSignOutAlt } from 'react-icons/fa';

const Navbar = () => {
    const { token, logout } = React.useContext(AuthContext);
    const navigate = useNavigate();

    const handleLogout = () => {
        logout();
        navigate('/login');
    };

    return (
        <nav className="bg-white shadow">
            <div className="container mx-auto px-4 py-4 flex justify-between items-center">
                <Link to="/" className="flex items-center text-xl font-bold text-blue-600">
                    <FaBook className="mr-2" />
                    Smart Study Planner
                </Link>
                <div className="flex items-center space-x-4">
                    {token ? (
                        <>
                            <Link to="/dashboard" className="flex items-center text-gray-700 hover:text-blue-600">
                                <FaUser className="mr-1" />
                                Dashboard
                            </Link>
                            <button onClick={handleLogout} className="flex items-center text-gray-700 hover:text-red-600">
                                <FaSignOutAlt className="mr-1" />
                                Logout
                            </button>
                        </>
                    ) : (
                        <>
                            <Link to="/login" className="flex items-center text-gray-700 hover:text-blue-600">
                                <FaUser className="mr-1" />
                                Login
                            </Link>
                            <Link to="/register" className="flex items-center text-gray-700 hover:text-blue-600">
                                <FaUser className="mr-1" />
                                Register
                            </Link>
                        </>
                    )}
                </div>
            </div>
        </nav>
    );
};

export default Navbar;
