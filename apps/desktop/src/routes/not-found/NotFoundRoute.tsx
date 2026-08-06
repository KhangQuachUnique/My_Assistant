import { Link } from "react-router";

export function NotFoundRoute() {
  return (
    <section className="grid w-full place-items-center">
      <div className="max-w-md text-center">
        <p className="text-sm font-semibold uppercase tracking-normal text-teal-300">
          404
        </p>
        <h2 className="mt-3 text-3xl font-semibold text-white">
          Route khong ton tai
        </h2>
        <p className="mt-4 text-zinc-300">
          Duong dan nay chua duoc khai bao trong route tree.
        </p>
        <Link
          to="/"
          className="mt-6 inline-flex rounded-lg bg-teal-300 px-4 py-2.5 text-sm font-semibold text-zinc-950 transition hover:bg-teal-200"
        >
          Ve Home
        </Link>
      </div>
    </section>
  );
}
