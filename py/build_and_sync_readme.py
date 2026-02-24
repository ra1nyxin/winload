import shutil
import subprocess
import os

# Paths
root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
py_dir = os.path.abspath(os.path.dirname(__file__))
readme_src = os.path.join(root_dir, 'README.md')
readme_dst = os.path.join(py_dir, 'README.md')
pyproject_path = os.path.join(py_dir, 'pyproject.toml')

# 1. Copy README.md from root to py/
if os.path.exists(readme_src):
    shutil.copyfile(readme_src, readme_dst)
    print(f"✅ Copied {readme_src} to {readme_dst}")
else:
    print(f"❌ {readme_src} does not exist!")

# 2. Update pyproject.toml readme field to 'README.md'
with open(pyproject_path, 'r', encoding='utf-8') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if line.strip().startswith('readme ='):
        lines[i] = 'readme = "README.md"\n'

with open(pyproject_path, 'w', encoding='utf-8') as f:
    f.writelines(lines)
print("✅ Updated pyproject.toml readme field to 'README.md'")

# 3. Run 'uv build'
try:
    result = subprocess.run(['uv', 'build'], cwd=py_dir, check=True)
    print("✅ uv build succeeded")
except subprocess.CalledProcessError as e:
    print(f"❌ uv build failed: {e}")
