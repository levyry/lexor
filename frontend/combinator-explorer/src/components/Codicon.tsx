import type { JSX } from 'preact';

interface CodiconProps extends JSX.HTMLAttributes<HTMLSpanElement> {
    name: string;
    className?: string;
}

export const Codicon = ({ name, className = '', ...props }: CodiconProps) => {
    return <span className={`codicon codicon-${name} ${className}`} {...props} />;
};
