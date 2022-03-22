use crate::utility::{install_deps, string_to_file, CustomResult};

const TS_TSCONFIG: &str = r##"{
  "compilerOptions": {
    "module": "CommonJS",
    "target": "ES6",
    "outDir": "dist",
    "sourceMap": true,
    "strict": true,
    "resolveJsonModule": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "baseUrl": "src"
  },
  "exclude": ["node_modules"],
  "include": ["src/**/*.ts"]
}
"##;
const TSX_TSCONFIG: &str = r##"{
  "compilerOptions": {
    "outDir": "./build",
    "allowJs": true,
    "target": "es6",
    "lib": ["dom", "dom.iterable", "esnext"],
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": ["./src/**/*"]
}
"##;

pub fn run(setup_type: &str) {
  migrate_install_deps(setup_type).unwrap();
  let content = if setup_type.eq("ts") {
    TS_TSCONFIG
  } else {
    TSX_TSCONFIG
  };

  string_to_file(content, &"tsconfig.json".to_string()).unwrap();
}

fn migrate_install_deps(setup_type: &str) -> CustomResult<()> {
  let deps = if setup_type.eq("ts") {
    vec!["typescript"]
  } else {
    vec!["typescript"]
  };

  install_deps(deps)
}
