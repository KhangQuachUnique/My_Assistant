import { NavLink, Outlet } from "react-router";

const navigationItems = [{ label: "Home", to: "/" }];

export function RootLayout() {
  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100">
      <div className="mx-auto flex min-h-screen w-full max-w-6xl flex-col px-5 py-5 sm:px-8">
        <header className="flex items-center justify-between gap-4 border-b border-white/10 pb-5">
          <div>
            <p className="text-xs font-semibold uppercase tracking-normal text-teal-300">
              AI Desktop Assistant
            </p>
            <h1 className="mt-1 text-2xl font-semibold text-white">
              Frontend foundation
            </h1>
          </div>

          <nav className="flex rounded-lg border border-white/10 bg-white/5 p-1">
            {navigationItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                end
                className={({ isActive }) =>
                  [
                    "rounded-md px-3 py-2 text-sm font-medium transition",
                    isActive
                      ? "bg-teal-300 text-zinc-950"
                      : "text-zinc-300 hover:bg-white/10 hover:text-white",
                  ].join(" ")
                }
              >
                {item.label}
              </NavLink>
            ))}
          </nav>
        </header>

        <main className="flex flex-1 py-8">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
