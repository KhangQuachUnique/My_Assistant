export function HomeRoute() {
  return (
    <section className="grid w-full content-center">
      <div className="max-w-3xl">
        <p className="text-sm font-semibold uppercase tracking-normal text-teal-300">
          Ready for product routes
        </p>
        <h2 className="mt-3 text-4xl font-semibold leading-tight text-white sm:text-5xl">
          Khung React sơ khai cho Tèo desktop.
        </h2>
        <p className="mt-5 text-base leading-7 text-zinc-300">
          Frontend hiện chỉ giữ app shell, route foundation và Tailwind entrypoint.
          Feature, API adapter và UI primitives sẽ được thêm khi có workflow thật.
        </p>
      </div>
    </section>
  );
}
