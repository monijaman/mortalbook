import Link from 'next/link';
import React from 'react';
import { Memorial } from '../lib/types';

interface MemorialCardProps {
    memorial: Memorial;
}

export const MemorialCard: React.FC<MemorialCardProps> = ({ memorial }) => {
    const birthYear = memorial.date_of_birth ? new Date(memorial.date_of_birth).getFullYear() : '?';
    const deathYear = memorial.date_of_death ? new Date(memorial.date_of_death).getFullYear() : '?';

    return (
        <Link href={`/memorials/${memorial.id}`}>
            <div className="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow overflow-hidden cursor-pointer">
                <div className="bg-gradient-to-r from-gray-300 to-gray-400 h-48 flex items-center justify-center">
                    <span className="text-gray-600 text-4xl">📷</span>
                </div>
                <div className="p-6">
                    <h3 className="text-xl font-semibold text-gray-900 mb-2">
                        {memorial.name || 'Unnamed'}
                    </h3>
                    <p className="text-sm text-gray-600 mb-4">
                        {birthYear} - {deathYear}
                    </p>
                    <p className="text-gray-700 text-sm line-clamp-3">
                        {memorial.biography || 'No biography available'}
                    </p>
                </div>
            </div>
        </Link>
    );
};

export default MemorialCard;
