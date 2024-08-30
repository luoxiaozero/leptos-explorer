const esbuild = require("esbuild");
const path = require("path");
const glob = require("glob");
const fs = require("fs");

const production = process.argv.includes("--production");
const watch = process.argv.includes("--watch");

/**
 * @type {import('esbuild').Plugin}
 */
const esbuildCopyNodeFilePlugin = {
    name: "esbuild-copy-node-file",

    setup(build) {
        // if (production) {
        //     return;
        // }
        build.onEnd(async (result) => {
            const outputFile = path.resolve(build.initialOptions.outfile);
            const outputDir = path.dirname(outputFile);
            glob.glob("leptos-explorer/*.node")
                .then((files) => {
                    files.forEach((filePath) => {
                        const fileName = path.basename(filePath);
                        const destPath = path.join(outputDir, fileName);

                        fs.copyFile(filePath, destPath, (err) => {
                            if (err) throw err;
                            console.log(
                                `${filePath} was copied to ${destPath}`
                            );
                        });
                    });
                })
                .catch((err) => {
                    throw err;
                });
        });
    },
};

/**
 * @type {import('esbuild').Plugin}
 */
const esbuildProblemMatcherPlugin = {
    name: "esbuild-problem-matcher",

    setup(build) {
        build.onStart(() => {
            console.log("[watch] build started");
        });
        build.onEnd((result) => {
            result.errors.forEach(({ text, location }) => {
                console.error(`✘ [ERROR] ${text}`);
                console.error(
                    `    ${location.file}:${location.line}:${location.column}:`
                );
            });
            console.log("[watch] build finished");
        });
    },
};

async function main() {
    const ctx = await esbuild.context({
        entryPoints: ["src/extension.ts"],
        bundle: true,
        format: "cjs",
        minify: production,
        sourcemap: !production,
        sourcesContent: false,
        platform: "node",
        outfile: "dist/extension.js",
        external: ["vscode", "*.node"],
        logLevel: "silent",
        loader: {
            ".node": "file",
        },
        plugins: [
            esbuildCopyNodeFilePlugin,
            /* add to the end of plugins array */
            esbuildProblemMatcherPlugin,
        ],
    });
    if (watch) {
        await ctx.watch();
    } else {
        await ctx.rebuild();
        await ctx.dispose();
    }
}

main().catch((e) => {
    console.error(e);
    process.exit(1);
});
