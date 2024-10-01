import React, { useEffect, useState, useContext } from 'react';
import api from '../api';
import { AuthContext } from '../contexts/AuthContext';

const Dashboard = () => {
    const { token, logout } = useContext(AuthContext);
    const [progress, setProgress] = useState(null);
    const [subjects, setSubjects] = useState([]);
    const [profile, setProfile] = useState(null);
    const [tasks, setTasks] = useState([]);
    const [studySessions, setStudySessions] = useState([]);
    const [error, setError] = useState('');

    useEffect(() => {
        const fetchData = async () => {
            try {
                //  profile
                const profileResponse = await api.get('/profile');
                setProfile(profileResponse.data);

                //  progress
                const progressResponse = await api.get('/progress');
                setProgress(progressResponse.data);

                //  subjects
                const subjectsResponse = await api.get('/subjects');
                setSubjects(subjectsResponse.data);

                //  tasks
                const tasksResponse = await api.get('/tasks');
                setTasks(tasksResponse.data);

                //  study sessions
                const sessionsResponse = await api.get('/study-sessions');
                setStudySessions(sessionsResponse.data);

            } catch (err) {
                setError('Failed to fetch data.');
                console.error(err);
            }
        };

        fetchData();
    }, [token]);

    const handleLogout = () => {
        logout();
    };

    if (error) {
        return <div className="container mx-auto p-4"><p className="text-red-500">{error}</p></div>;
    }

    if (!profile || !progress) {
        return <div className="container mx-auto p-4"><p>Loading...</p></div>;
    }

    return (
        <div className="container mx-auto p-4">
            <div className="flex justify-between items-center mb-6">
                <h1 className="text-3xl font-bold">Welcome, {profile.username}!</h1>
                <button
                    onClick={handleLogout}
                    className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                >
                    Logout
                </button>
            </div>

            {}
            <section className="mb-6">
                <h2 className="text-2xl font-semibold mb-2">Your Progress</h2>
                <p>Completed Tasks: {progress.completed_tasks}</p>
                <p>Total Tasks: {progress.total_tasks}</p>
            </section>

            {}
            <section className="mb-6">
                <h2 className="text-2xl font-semibold mb-2">Your Subjects</h2>
                <ul className="space-y-2">
                    {subjects.map(subject => (
                        <li key={subject.id} className="p-4 bg-gray-100 rounded">
                            <h3 className="text-xl font-bold">{subject.name}</h3>
                            <p>{subject.description || 'No description provided.'}</p>
                        </li>
                    ))}
                </ul>
            </section>

            {}
            <section className="mb-6">
                <h2 className="text-2xl font-semibold mb-2">Your Tasks</h2>
                <ul className="space-y-2">
                    {tasks.map(task => (
                        <li key={task.id} className="p-4 bg-gray-100 rounded">
                            <h3 className="text-xl font-bold">{task.title}</h3>
                            <p>{task.description || 'No description provided.'}</p>
                            <p>Deadline: {task.deadline || 'No deadline set.'}</p>
                            <p>Difficulty Level: {task.difficulty_level || 'N/A'}</p>
                        </li>
                    ))}
                </ul>
            </section>

            {}
            <section className="mb-6">
                <h2 className="text-2xl font-semibold mb-2">Your Study Sessions</h2>
                <ul className="space-y-2">
                    {studySessions.map(session => (
                        <li key={session.id} className="p-4 bg-gray-100 rounded">
                            <p>Scheduled At: {session.scheduled_at}</p>
                            <p>Duration: {session.duration} minutes</p>
                            <p>Status: {session.completed ? 'Completed' : 'Pending'}</p>
                        </li>
                    ))}
                </ul>
            </section>
        </div>
    );
};

export default Dashboard;
